use super::{overlay::VideoDetailsOverlay, PostViewCtx};
use crate::{
    canister::{
        individual_user_template::PostViewDetailsFromFrontend,
        utils::{bg_url, mp4_url},
    },
    component::feed_popup::FeedPopUp,
    state::{
        auth::account_connected_reader, canisters::unauth_canisters,
        local_storage::use_referrer_store,
    },
};
use leptos::{html::Video, *};
use leptos_use::{use_event_listener, watch_debounced};

#[component]
pub fn BgView(idx: usize, children: Children) -> impl IntoView {
    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();
    let post = move || video_queue.with(|q| q.get(idx).cloned());
    let uid = move || post().as_ref().map(|q| q.uid.clone()).unwrap_or_default();

    let (is_connected, _) = account_connected_reader();
    let (show_login_popup, set_show_login_popup) = create_signal(true);

    let (show_refer_login_popup, set_show_refer_login_popup) = create_signal(true);
    let (referrer_store, _, _) = use_referrer_store();

    create_effect(move |_| {
        if current_idx.get() == 5 {
            set_show_login_popup.update(|n| *n = false);
        }
        Some(())
    });

    view! {
        <div class="bg-transparent w-full h-full relative overflow-hidden">
            <div
                class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                style:background-color="rgb(0, 0, 0)"
                style:background-image=move || format!("url({})", bg_url(uid()))
            ></div>
            <Show when=move || { idx == 4 && !is_connected.get() && show_login_popup.get() }>
                <FeedPopUp
                    on_click=move |_| set_show_login_popup.set(false)
                    header_text="Your Rewards are
                    Waiting!"
                    body_text="SignUp/Login to save your progress and claim your rewards."
                    login_text="Login"
                />
            </Show>
            <Show when=move || {
                referrer_store.get().is_some() && idx == 0 && !is_connected.get()
                    && show_refer_login_popup.get()
            }>
                <FeedPopUp
                    on_click=move |_| set_show_refer_login_popup.set(false)
                    header_text="Claim Your Referral
                    Rewards Now!"
                    body_text="SignUp from this link to get 500 COYNs as referral rewards."
                    login_text="Sign Up"
                />
            </Show>
            {move || post().map(|post| view! { <VideoDetailsOverlay post/> })}
            {children()}
        </div>
    }
}

#[component]
pub fn VideoView(idx: usize, muted: RwSignal<bool>) -> impl IntoView {
    let container_ref = create_node_ref::<Video>();
    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();

    let uid =
        create_memo(move |_| with!(|video_queue| video_queue.get(idx).map(|q| q.uid.clone())));
    let view_bg_url = move || uid().map(bg_url);
    let view_video_url = move || uid().map(mp4_url);

    // Handles autoplay
    create_effect(move |_| {
        let Some(vid) = container_ref() else {
            return;
        };
        if idx != current_idx() {
            _ = vid.pause();
            return;
        }
        vid.set_autoplay(true);
        _ = vid.play();
    });

    // Handles mute/unmute
    create_effect(move |_| {
        let vid = container_ref()?;
        vid.set_muted(muted());
        Some(())
    });

    create_effect(move |_| {
        let vid = container_ref()?;
        // the attributes in DOM don't seem to be working
        vid.set_muted(muted.get_untracked());
        vid.set_loop(true);
        Some(())
    });

    let watched_percentage_signal = create_rw_signal(0_u8);
    let watched_count_signal = create_rw_signal(0_u8);

    let mut video_previous_current_time = 0.0;
    let _ = use_event_listener(container_ref, ev::timeupdate, move |event| {
        if let Some(video) = container_ref() {
            let duration = video.duration();
            let current_time = video.current_time();

            if current_time < video_previous_current_time {
                watched_count_signal.update(|watched_count| *watched_count += 1);
            }
            watched_percentage_signal.update(|watched_percentage| {
                *watched_percentage = (100.0 * (current_time / duration)) as u8;
            });
            video_previous_current_time = current_time;
        }
    });

    let post = video_queue.with_untracked(|vq| vq.get(idx).cloned());
    let canister_id = post.as_ref().map(|post| post.canister_id).unwrap();
    let post_id = post.as_ref().map(|post| post.post_id).unwrap();

    let send_view_detail_action = create_action(move |()| async move {
        let canisters = unauth_canisters();
        let watched_percentage = watched_percentage_signal.get_untracked();
        let watched_count = watched_count_signal.get_untracked();
        let payload = match watched_count {
            0 => PostViewDetailsFromFrontend::WatchedPartially {
                percentage_watched: watched_percentage,
            },
            _ => PostViewDetailsFromFrontend::WatchedMultipleTimes {
                percentage_watched: watched_percentage,
                watch_count: watched_count,
            },
        };
        let send_view_res = canisters
            .individual_user(canister_id)
            .update_post_add_view_details(post_id, payload)
            .await;

        match send_view_res {
            Ok(()) => {}
            Err(e) => log::warn!("failed to send view details: {:?}", e),
        }

        watched_count_signal.update(|watched_count| *watched_count = 0);
        watched_percentage_signal.update(|watched_percentage| *watched_percentage = 0);
    });

    let send_view_details_guard = create_memo(move |_| {
        current_idx() != idx && (watched_percentage_signal() != 0 || watched_count_signal() != 0)
    });

    let _ = watch_debounced(
        watched_percentage_signal,
        move |_, _, _| {
            if send_view_details_guard() {
                send_view_detail_action.dispatch(());
            }
        },
        2000.0,
    );

    view! {
        <label class="w-full h-full absolute top-0 left-0 grid grid-cols-1 justify-items-center items-center cursor-pointer z-[3]">
            <input
                on:change=move |_| muted.update(|m| *m = !*m)
                type="checkbox"
                value=""
                class="sr-only"
            />
            <video
                _ref=container_ref
                class="object-contain h-dvh max-h-dvh cursor-pointer"
                poster=view_bg_url
                src=view_video_url
                loop
                muted
                playsinline
                disablepictureinpicture
                disableremoteplayback
                preload="auto"
            ></video>
        </label>
    }
}
