use crate::{
    canister::utils::{bg_url, mp4_url},
    state::canisters::{Canisters, CanistersError},
    utils::{profile::ProfileDetails, MockPartialEq},
};
use candid::Principal;
use leptos::{html::Video, *};

use super::{overlay::VideoDetailsOverlay, PostViewCtx};

#[component]
pub fn BgView(idx: usize, children: Children) -> impl IntoView {
    let PostViewCtx { video_queue, .. } = expect_context();
    let post = move || video_queue.with(|q| q.get(idx).cloned());
    let uid = move || post().as_ref().map(|q| q.uid.clone()).unwrap_or_default();
    view! {
        <div class="bg-transparent w-full h-full relative overflow-hidden">
            <div
                class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                style:background-color="rgb(0, 0, 0)"
                style:background-image=move || format!("url({})", bg_url(uid()))
            ></div>
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

    let profile_and_canister_details: Resource<
        MockPartialEq<Option<Result<Canisters<true>, CanistersError>>>,
        Option<(ProfileDetails, Principal)>,
    > = expect_context();

    let vid_details =
        create_memo(move |_| with!(|video_queue| video_queue.get(idx).map(|q| q.clone())));

    let publisher_user_id = move || vid_details().as_ref().map(|q| q.poster_principal);
    let user_id = move || {
        profile_and_canister_details()
            .flatten()
            .map(|(q, _)| q.principal)
    };
    let is_loggedin = move || user_id().is_some();
    let display_name = move || {
        profile_and_canister_details()
            .flatten()
            .map(|(q, _)| q.display_name)
    };
    let canister_id = move || profile_and_canister_details().flatten().map(|(_, q)| q);
    let video_id = move || vid_details().as_ref().map(|q| q.uid.clone());
    let hastag_count = move || vid_details().as_ref().map(|q| q.hastags.len());
    let is_nsfw = move || vid_details().as_ref().map(|q| q.is_nsfw);
    let is_hotornot = move || {
        vid_details()
            .as_ref()
            .map(|q| q.hot_or_not_feed_ranking_score.is_some())
    };
    let view_count = move || vid_details().as_ref().map(|q| q.views);
    let like_count = move || vid_details().as_ref().map(|q| q.likes);

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

    // video_viewed - analytics
    #[cfg(feature = "hydrate")]
    {
        use crate::utils::event_streaming::send_event;
        use serde_json::json;
        use wasm_bindgen::JsCast;

        let (video_watched, set_video_watched) = create_signal(false);

        create_effect(move |_| {
            let vid = container_ref()?;

            let callback = move |e: web_sys::Event| {
                if video_watched.get() {
                    return;
                }

                let target = e.target().unwrap();
                let video = target.unchecked_into::<web_sys::HtmlVideoElement>();
                // let duration = video.duration() as f64;
                let current_time = video.current_time() as f64;

                if current_time >= 3.0 {
                    // Video is halfway done, take action here

                    send_event(
                        "video_viewed",
                        &json!({
                            "publisher_user_id":publisher_user_id(),
                            "user_id":user_id(),
                            "is_loggedIn": is_loggedin(),
                            "display_name": display_name(),
                            "canister_id": canister_id(),
                            "video_id": video_id(),
                            "video_category": "NA",
                            "creator_category": "NA",
                            "hashtag_count": hastag_count(),
                            "is_NSFW": is_nsfw(),
                            "is_hotorNot": is_hotornot(),
                            "feed_type": "NA",
                            "view_count": view_count(),
                            "like_count": like_count(),
                            "share_count": 0,
                        }),
                    );
                    set_video_watched.set(true);
                }
            };

            let _ = vid.on(ev::timeupdate, callback);

            Some(())
        });
    }

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
