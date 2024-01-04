use std::ops::Deref;

use candid::Principal;
use leptos::{html::Video, *};
use leptos_router::*;

use crate::{component::spinner::Spinner, js::wasp::WaspHlsPlayer, state::canisters::Canisters};

async fn get_post_uid(canisters: &Canisters, user_canister: Principal, post_id: u64) -> String {
    let post_creator_can = canisters.individual_user(user_canister);
    // TODO: error handling
    let post_details = post_creator_can
        .get_individual_post_details_by_id(post_id)
        .await
        .unwrap();
    post_details.video_uid
}

#[derive(Params, PartialEq)]
struct PostParams {
    canister_id: String,
    post_id: u64,
}

#[component]
pub fn PostView() -> impl IntoView {
    let params = use_params::<PostParams>();
    let canister_and_post = move || {
        params.with(|p| {
            let go_to_root = || {
                let nav = use_navigate();
                nav("/", Default::default());
                None
            };
            let Ok(p) = p else {
                return go_to_root();
            };
            let Ok(canister_id) = Principal::from_text(&p.canister_id) else {
                return go_to_root();
            };

            Some((canister_id, p.post_id))
        })
    };

    let video_uid = create_resource(canister_and_post, |can_and_post| async move {
        let canister = expect_context::<Canisters>();
        let Some((canister_id, post_id)) = can_and_post else {
            return String::new();
        };
        get_post_uid(&canister, canister_id, post_id).await
    });

    let video_ref = create_node_ref::<Video>();
    let (wasp_player, set_wasp_player) = create_signal(None::<WaspHlsPlayer>);

    create_effect(move |_| {
        let Some(video) = video_ref.get() else {
            return;
        };
        let wasp = WaspHlsPlayer::new(&video, None);
        set_wasp_player(Some(wasp));
        video.deref().set_muted(true);
        video.deref().set_loop(true);
    });

    view! {
        <Suspense fallback=|| {
            view! {
                <div class="grid grid-cols-1 h-screen w-screen bg-black justify-items-center place-content-center">
                    <Spinner/>
                </div>
            }
        }>
            {move || {
                video_uid
                    .get()
                    .map(|uid| {
                        let bgurl = format!(
                            "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/thumbnails/thumbnail.jpg",
                            uid,
                        );
                        create_effect(move |_| {
                            let url = format!(
                                "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/manifest/video.m3u8",
                                uid,
                            );
                            wasp_player
                                .with(|wasp| {
                                    let Some(wasp) = wasp else {
                                        return;
                                    };
                                    wasp.load(&url);
                                });
                        });
                        view! {
                            <div
                                class="bg-black bg-cover"
                                style:background-image=move || format!("url({})", bgurl)
                            >
                                <div class="grid grid-cols-1 h-screen w-screen justify-items-center backdrop-blur-lg">
                                    <video
                                        _ref=video_ref
                                        class="object-contain h-screen"
                                        loop
                                        autoplay
                                        muted
                                    ></video>
                                </div>
                            </div>
                        }
                    })
            }}

        </Suspense>
    }
}
