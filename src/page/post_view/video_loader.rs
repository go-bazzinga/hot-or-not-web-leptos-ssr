use crate::canister::utils::{bg_url, mp4_url};
use leptos::{html::Video, *};

use super::PostViewCtx;

#[component]
pub fn BgView(uid: String, children: Children) -> impl IntoView {
    view! {
        <div class="bg-transparent w-full h-full relative">
            <div
                class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                style:background-color="rgb(0, 0, 0)"
                style:background-image=move || format!("url({})", bg_url(&uid))
            ></div>
            <div class="grid grid-cols-1 h-full w-full justify-items-center bg-transparent absolute top-0 left-0 z-[2]">
                {children()}
            </div>
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
        let vid = container_ref().unwrap();
        if idx != current_idx() {
            _ = vid.pause();
            return;
        }
        vid.set_autoplay(true);
        _ = vid.play();
    });

    // Handles mute/unmute
    create_effect(move |_| {
        let vid = container_ref().unwrap();
        vid.set_muted(muted());
    });

    create_effect(move |_| {
        let vid = container_ref().unwrap();
        // the attributes in DOM don't seem to be working
        vid.set_muted(muted.get_untracked());
        vid.set_loop(true);
    });

    view! {
        <video
            on:click=move |_| muted.update(|m| *m = !*m)
            _ref=container_ref
            class="object-contain absolute z-[3] h-dvh max-h-dvh cursor-pointer"
            poster=view_bg_url
            src=view_video_url
            loop
            muted
            playsinline
            disablepictureinpicture
            disableremoteplayback
            preload="auto"
        ></video>
    }
}
