pub mod error;
pub mod overlay;
pub mod video_iter;
pub mod video_loader;
use crate::state::audio_state::AudioState;
use crate::{
    component::{scrolling_post_view::ScrollingPostView, spinner::FullScreenSpinner},
    consts::NSFW_TOGGLE_STORE,
    state::canisters::{unauth_canisters, Canisters},
    try_or_redirect,
    utils::{
        posts::{get_post_uid, FetchCursor, PostDetails},
        route::failure_redirect,
    },
};
use candid::Principal;
use futures::StreamExt;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::{
    storage::use_local_storage, use_debounce_fn, use_intersection_observer_with_options,
    utils::FromToStringCodec, UseIntersectionObserverOptions,
};
use video_iter::VideoFetchStream;
use video_loader::{BgView, VideoView};

use overlay::HomeButtonOverlay;

#[derive(Params, PartialEq, Clone, Copy)]
struct PostParams {
    canister_id: Principal,
    post_id: u64,
}

#[derive(Clone, Default)]
pub struct PostViewCtx {
    fetch_cursor: RwSignal<FetchCursor>,
    // TODO: this is a dead simple with no GC
    // We're using virtual lists for DOM, so this doesn't consume much memory
    // as uids only occupy 32 bytes each
    // but ideally this should be cleaned up
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    queue_end: RwSignal<bool>,
}

// Infinite Scrolling View
// Basically a virtual list with 5 items visible at a time
#[component]
pub fn ScrollingView<NV: Fn() -> NVR + Clone + 'static, NVR>(
    next_videos: NV,
    recovering_state: RwSignal<bool>,
) -> impl IntoView {
    //TODO: take this as a parameter.
    let PostViewCtx {
        video_queue,
        current_idx,
        queue_end,
        ..
    } = expect_context();

    let AudioState {
        muted,
        show_mute_icon,
    } = AudioState::get();

    let scroll_root: NodeRef<html::Div> = create_node_ref::<html::Div>();

    //LEARN: This creates scrolling view which will be used for intersection observer.

    view! {
        <div class="h-full w-full overflow-hidden overflow-y-auto">
            <div
                _ref=scroll_root
                class="snap-mandatory snap-y overflow-y-scroll h-dvh w-dvw bg-black"
                style:scroll-snap-points-y="repeat(100vh)"
            >
                <HomeButtonOverlay />
                <For
                    each=move || video_queue().into_iter().enumerate()
                    key=|(_, details)| (details.canister_id, details.post_id)
                    children=move |(queue_idx, _details)| {
                        let container_ref = create_node_ref::<html::Div>();
                        let next_videos = next_videos.clone();
                        use_intersection_observer_with_options(
                            container_ref,
                            move |entry, _| {
                                let Some(visible) = entry
                                    .first()
                                    .filter(|entry| entry.is_intersecting()) else {
                                    return;
                                };
                                let rect = visible.bounding_client_rect();
                                if rect.y() == rect.height()
                                    || queue_idx == current_idx.get_untracked()
                                {
                                    return;
                                }
                                if video_queue.with_untracked(|q| q.len()).saturating_sub(queue_idx)
                                    <= 10
                                {
                                    next_videos();
                                }
                                current_idx.set(queue_idx);
                            },
                            UseIntersectionObserverOptions::default()
                                .thresholds(vec![0.83])
                                .root(Some(scroll_root)),
                        );
                        create_effect(move |_| {
                            let Some(container) = container_ref() else {
                                return;
                            };
                            if current_idx.get_untracked() == queue_idx
                                && recovering_state.get_untracked()
                            {
                                container.scroll_into_view();
                                recovering_state.set(false);
                            }
                        });
                        let show_video = create_memo(move |_| {
                            queue_idx.abs_diff(current_idx()) <= 20
                        });
                        view! {
                            <div _ref=container_ref class="snap-always snap-end w-full h-full">
                                <Show when=show_video>
                                    <BgView video_queue current_idx idx=queue_idx>
                                        <VideoView video_queue current_idx idx=queue_idx muted />
                                    </BgView>
                                </Show>
                            </div>
                        }
                    }
                />

                <Show when=queue_end>
                    <div class="h-full w-full relative top-0 left-0 bg-inherit z-[21] flex snap-always snap-end justify-center items-center text-xl text-white/80">
                        <span>You have reached the end!</span>
                    </div>
                </Show>

                <Show when=show_mute_icon>
                    <button
                        class="fixed top-1/2 left-1/2 z-20 cursor-pointer"
                        on:click=move |_| muted.set(false)
                    >
                        <Icon
                            class="text-white/80 animate-ping text-4xl"
                            icon=icondata::BiVolumeMuteSolid
                        />
                    </button>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn PostViewWithUpdates(initial_post: Option<PostDetails>) -> impl IntoView {
    let PostViewCtx {
        fetch_cursor,
        video_queue,
        current_idx,
        queue_end,
    } = expect_context();

    let recovering_state = create_rw_signal(false);
    if let Some(initial_post) = initial_post.clone() {
        fetch_cursor.update_untracked(|f| {
            // we've already fetched the first posts
            if f.start > 1 || queue_end.get_untracked() {
                recovering_state.set(true);
                return;
            }
            f.start = 1;
            f.limit = 1;
        });
        video_queue.update_untracked(|v| {
            if v.len() > 1 {
                // Safe to do a GC here
                let rem = 0..(current_idx.get_untracked().saturating_sub(6));
                current_idx.update(|c| *c -= rem.len());
                v.drain(rem);
                return;
            }
            *v = vec![initial_post];
        })
    }
    let (nsfw_enabled, _, _) = use_local_storage::<bool, FromToStringCodec>(NSFW_TOGGLE_STORE);
    let auth_canisters: RwSignal<Option<Canisters<true>>> = expect_context();

    let fetch_video_action = create_action(move |_| async move {
        loop {
            let Some(cursor) = fetch_cursor.try_get_untracked() else {
                return;
            };
            let auth_canisters = auth_canisters.get_untracked();
            let nsfw_enabled = nsfw_enabled.get_untracked();
            let unauth_canisters = unauth_canisters();

            let chunks = if let Some(canisters) = auth_canisters.as_ref() {
                let fetch_stream = VideoFetchStream::new(canisters, cursor);
                fetch_stream.fetch_post_uids_chunked(3, nsfw_enabled).await
            } else {
                let fetch_stream = VideoFetchStream::new(&unauth_canisters, cursor);
                fetch_stream.fetch_post_uids_chunked(3, nsfw_enabled).await
            };

            let res = try_or_redirect!(chunks);
            let mut chunks = res.posts_stream;
            let mut cnt = 0;
            while let Some(chunk) = chunks.next().await {
                cnt += chunk.len();
                video_queue.try_update(|q| {
                    for uid in chunk {
                        let uid = try_or_redirect!(uid);
                        q.push(uid);
                    }
                });
            }
            if res.end || cnt >= 8 {
                queue_end.try_set(res.end);
                break;
            }
            fetch_cursor.try_update(|c| c.advance());
        }

        fetch_cursor.try_update(|c| c.advance());
    });
    create_effect(move |_| {
        if !recovering_state.get_untracked() {
            fetch_video_action.dispatch(());
        }
    });
    let next_videos = use_debounce_fn(
        move || {
            if !fetch_video_action.pending().get_untracked() && !queue_end.get_untracked() {
                fetch_video_action.dispatch(())
            }
        },
        500.0,
    );

    let current_post_base = create_memo(move |_| {
        with!(|video_queue| {
            let cur_idx = current_idx();
            let details = video_queue.get(cur_idx)?;
            Some((details.canister_id, details.post_id))
        })
    });

    create_effect(move |_| {
        let Some((canister_id, post_id)) = current_post_base() else {
            return;
        };
        use_navigate()(
            &format!("/hot-or-not/{canister_id}/{post_id}",),
            Default::default(),
        );
    });

    view! {
        <ScrollingPostView
            video_queue
            current_idx
            recovering_state
            fetch_next_videos=next_videos
            queue_end
            overlay=|| view! { <HomeButtonOverlay /> }
        />
    }
}

#[component]
pub fn PostView() -> impl IntoView {
    let params = use_params::<PostParams>();
    let initial_canister_and_post = create_rw_signal(params.get_untracked().ok());

    create_effect(move |_| {
        if initial_canister_and_post.with_untracked(|p| p.is_some()) {
            return None;
        }
        let p = params.get().ok()?;
        initial_canister_and_post.set(Some(p));
        Some(())
    });

    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();
    let canisters = unauth_canisters();

    let fetch_first_video_uid = create_resource(initial_canister_and_post, move |params| {
        let canisters = canisters.clone();
        async move {
            let Some(params) = params else {
                return Err(());
            };
            let cached_post = video_queue
                .with_untracked(|q| q.get(current_idx.get_untracked()).cloned())
                .filter(|post| {
                    post.canister_id == params.canister_id && post.post_id == params.post_id
                });
            if let Some(post) = cached_post {
                return Ok(Some(post));
            }

            match get_post_uid(&canisters, params.canister_id, params.post_id).await {
                Ok(post) => Ok(post),
                Err(e) => {
                    failure_redirect(e);
                    Err(())
                }
            }
        }
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                fetch_first_video_uid()
                    .and_then(|initial_post| {
                        let initial_post = initial_post.ok()?;
                        Some(view! { <PostViewWithUpdates initial_post /> })
                    })
            }}

        </Suspense>
    }
}
