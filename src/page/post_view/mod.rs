mod error;
mod overlay;
mod video_iter;
mod video_loader;

use candid::Principal;
use futures::StreamExt;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::{
    storage::use_local_storage, use_debounce_fn, use_intersection_observer_with_options,
    utils::FromToStringCodec, UseIntersectionObserverOptions,
};

use crate::{
    component::spinner::FullScreenSpinner,
    consts::NSFW_TOGGLE_STORE,
    state::canisters::{authenticated_canisters, unauth_canisters},
    try_or_redirect, try_or_redirect_opt,
    utils::{
        profile::ProfileDetails,
        route::{failure_redirect, go_to_root},
        MockPartialEq,
    },
};
use video_iter::{get_post_uid, VideoFetchStream};
use video_loader::{BgView, VideoView};

use overlay::HomeButtonOverlay;
use video_iter::{FetchCursor, PostDetails};

#[derive(Params, PartialEq)]
struct PostParams {
    canister_id: String,
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
}

// Infinite Scrolling View
// Basically a virtual list with 5 items visible at a time
#[component]
pub fn ScrollingView<NV: Fn() -> NVR + Clone + 'static, NVR>(
    next_videos: NV,
    recovering_state: RwSignal<bool>,
) -> impl IntoView {
    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();

    let muted = create_rw_signal(true);
    let scroll_root = create_node_ref::<html::Div>();

    view! {
        <div class="h-full w-full overflow-hidden overflow-y-auto">
            <div
                _ref=scroll_root
                class="snap-mandatory snap-y overflow-y-scroll h-dvh w-dvw bg-black"
                style:scroll-snap-points-y="repeat(100vh)"
            >
                <HomeButtonOverlay/>
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
                                    <BgView idx=queue_idx>
                                        <VideoView idx=queue_idx muted/>
                                    </BgView>
                                </Show>
                            </div>
                        }
                    }
                />

                <Show when=muted>
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
    } = expect_context();

    let recovering_state = create_rw_signal(false);
    if let Some(initial_post) = initial_post {
        fetch_cursor.update_untracked(|f| {
            // we've already fetched the first posts
            if f.start > 1 {
                recovering_state.set(true);
                return;
            }
            f.start = 1;
            f.limit = 1;
        });
        video_queue.update_untracked(|v| {
            if v.len() > 1 {
                // Safe to do a GC here
                let rem = 0..(current_idx.get_untracked().saturating_sub(3));
                current_idx.update(|c| *c -= rem.len());
                v.drain(rem);
                return;
            }
            *v = vec![initial_post];
        })
    }
    let (nsfw_enabled, _, _) = use_local_storage::<bool, FromToStringCodec>(NSFW_TOGGLE_STORE);
    let auth_cans_res = authenticated_canisters();

    let fetch_video_action = create_action(move |()| async move {
        loop {
            let auth_canisters = leptos::untrack(|| auth_cans_res.get().transpose());
            let auth_canisters = try_or_redirect!(auth_canisters).flatten();
            let cursor = fetch_cursor.get_untracked();
            let nsfw_enabled = nsfw_enabled.get_untracked();
            let unauth_canisters = unauth_canisters();

            let chunks = if let Some(canisters) = auth_canisters.as_ref() {
                let fetch_stream = VideoFetchStream::new(canisters, cursor);
                fetch_stream.fetch_post_uids_chunked(3, nsfw_enabled).await
            } else {
                let fetch_stream = VideoFetchStream::new(&unauth_canisters, cursor);
                fetch_stream.fetch_post_uids_chunked(3, nsfw_enabled).await
            };

            let mut chunks = try_or_redirect!(chunks);
            let mut cnt = 0;
            while let Some(chunk) = chunks.next().await {
                cnt += chunk.len();
                video_queue.update(|q| {
                    for uid in chunk {
                        let uid = try_or_redirect!(uid);
                        q.push(uid);
                    }
                });
            }
            if cnt < 8 {
                fetch_cursor.update(|c| c.advance());
            } else {
                break;
            }
        }

        fetch_cursor.update(|c| c.advance());
    });
    create_effect(move |_| {
        if !recovering_state.get_untracked() {
            fetch_video_action.dispatch(());
        }
    });
    let next_videos = use_debounce_fn(
        move || {
            if !fetch_video_action.pending().get_untracked() {
                log::debug!("trigger rerender");
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

    view! { <ScrollingView next_videos recovering_state/> }
}

#[component]
pub fn PostView() -> impl IntoView {
    // user profile details
    let canisters = authenticated_canisters();
    let profile_details = create_resource(
        move || MockPartialEq(canisters.get().and_then(|c| c.transpose())),
        move |canisters| async move {
            let canisters = try_or_redirect_opt!(canisters.0?);
            let user = canisters.authenticated_user();
            let user_details = user.get_profile_details().await.ok()?;
            Some((
                ProfileDetails::from(user_details),
                canisters.user_canister(),
            ))
        },
    );
    provide_context(profile_details);

    let params = use_params::<PostParams>();
    let canister_and_post = move || {
        params.with_untracked(|p| {
            let p = p.as_ref().ok()?;
            let canister_id = Principal::from_text(&p.canister_id).ok()?;

            Some((canister_id, p.post_id))
        })
    };

    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();

    let cached_post = move || {
        let Some((canister, post_id)) = canister_and_post() else {
            go_to_root();
            return None;
        };

        let post = video_queue
            .with_untracked(|q| q.get(current_idx.get_untracked()).cloned())
            .filter(|post| post.canister_id == canister && post.post_id == post_id);

        post
    };

    let fetch_first_video_uid = create_resource(
        || (),
        move |_| async move {
            let Some((canister, post_id)) = canister_and_post() else {
                go_to_root();
                return None;
            };

            let canisters = unauth_canisters();
            match get_post_uid(&canisters, canister, post_id).await {
                Ok(Some(uid)) => Some(uid),
                Err(e) => {
                    failure_redirect(e);
                    None
                }
                Ok(None) => None,
            }
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                if let Some(post) = cached_post() {
                    Some(view! { <PostViewWithUpdates initial_post=Some(post)/> })
                } else {
                    fetch_first_video_uid()
                        .map(|initial_post| {
                            view! { <PostViewWithUpdates initial_post=initial_post/> }
                        })
                }
            }}

        </Suspense>
    }
}
