use std::cell::Cell;

use crate::canister::utils::{bg_url, mp4_url};
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

    // Handle video half completed action
    #[cfg(feature = "hydrate")]
    {
        use gtag_js::DataLayer;
        use serde_json::json;
        use wasm_bindgen::JsCast;
        use web_sys::console;

        let (has_halfway_action_been_performed, set_has_halfway_action_been_performed) =
            create_signal(false);

        create_effect(move |_| {
            let vid = container_ref()?;

            let callback = move |e: web_sys::Event| {
                if has_halfway_action_been_performed.get() {
                    return;
                }

                let target = e.target().unwrap();
                let video = target.unchecked_into::<web_sys::HtmlVideoElement>();
                let duration = video.duration() as f64;
                let current_time = video.current_time() as f64;

                if current_time >= 3.0 {
                    // Video is halfway done, take action here

                    let gtag_res = DataLayer::new("G-R925ERNSQE");
                    let res = gtag_res.push_simple("video_viewed");
                    // if let Err(e) = res {
                    //     console::log_1(&format!("Error pushing to gtag: {}", e).into());
                    // }

                    console::log_1(&"video_viewed!".into());
                    set_has_halfway_action_been_performed.set(true);
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
