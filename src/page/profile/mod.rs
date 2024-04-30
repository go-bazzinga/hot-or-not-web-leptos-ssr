mod ic;
mod posts;
mod speculation;

use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use crate::{
    component::{back_btn::BackButton, connect::ConnectLogin, spinner::FullScreenSpinner},
    state::{auth::account_connected_reader, canisters::unauth_canisters},
    utils::profile::ProfileDetails,
};

use posts::ProfilePosts;
use speculation::ProfileSpeculationsPlaceHolder;

#[derive(Params, PartialEq)]
struct ProfileParams {
    id: String,
}

#[component]
fn Stat(stat: u64, #[prop(into)] info: String) -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col items-center text-white space-y-0.5">
            <span class="font-bold text-xl">{stat}</span>
            <span class="text-md">{info}</span>
        </div>
    }
}

#[component]
fn ListSwitcher(user_canister: Principal) -> impl IntoView {
    let (cur_tab, set_cur_tab) = create_query_signal::<String>("tab");
    let current_tab = create_memo(move |_| {
        with!(|cur_tab| match cur_tab.as_deref() {
            Some("posts") => 0,
            Some("speculations") => 1,
            _ => 0,
        })
    });
    let tab_class = move |tab_id: usize| {
        if tab_id == current_tab() {
            "text-primary-500 border-b-4 border-primary-500 flex justify-center w-full py-2"
        } else {
            "text-white flex justify-center w-full py-2"
        }
    };

    view! {
        <div class="relative flex flex-row w-7/12 text-center text-md md:text-lg lg:text-xl xl:text-2xl">
            <button class=move || tab_class(0) on:click=move |_| set_cur_tab(Some("posts".into()))>
                <Icon icon=icondata::FiGrid/>
            </button>
            <button
                class=move || tab_class(1)
                on:click=move |_| set_cur_tab(Some("speculations".into()))
            >
                <Icon icon=icondata::BsTrophy/>
            </button>
        </div>
        <div class="flex flex-col gap-y-12 justify-center pb-12 w-11/12 sm:w-7/12">
            <Show
                when=move || current_tab() == 0
                fallback=move || view! { <ProfileSpeculationsPlaceHolder/> }
            >
                <ProfilePosts user_canister/>
            </Show>
        </div>
    }
}

#[component]
fn ProfileViewInner(user: ProfileDetails, user_canister: Principal) -> impl IntoView {
    let username_or_principal = user.username_or_principal();
    let profile_pic = user.profile_pic_or_random();
    let display_name = user.display_name_or_fallback();
    let earnings = user.lifetime_earnings;
    let (is_connected, _) = account_connected_reader();

    view! {
        <div class="min-h-screen bg-black text-white overflow-y-scroll pt-10 pb-12">
            <div class="flex ml-4">
                <BackButton fallback="/".to_string()/>
            </div>
            <div class="grid grid-cols-1 gap-5 justify-normal justify-items-center w-full">
                <div
                    class="flex flex-row w-11/12 sm:w-7/12"
                    class:justify-center=is_connected
                    class:justify-between=move || !is_connected()
                >
                    <div class="flex flex-col items-center">
                        <img
                            class="h-24 w-24 rounded-full"
                            alt=username_or_principal.clone()
                            src=profile_pic
                        />
                        <div class="flex flex-col text-center">
                            <span class="text-md text-white font-bold">{display_name}</span>
                            <div class="text-sm flex flex-row">
                                <p class="text-white">@ {username_or_principal}</p>
                                <p class="text-white text-md font-bold px-1">{" • "}</p>
                                <p class="text-primary-500">{earnings} Earnings</p>
                            </div>
                        </div>
                    </div>
                    <Show when=move || !is_connected()>
                        <div class="md:w-4/12 w-6/12 pt-5 mr-5">
                            <ConnectLogin cta_location="profile"/>
                        </div>
                    </Show>
                </div>
                <div class="flex justify-around text-center rounded-full divide-x-2 divide-white/20 bg-white/10 p-4 my-4 w-11/12 sm:w-7/12">
                    <Stat stat=user.followers_cnt info="Lovers"/>
                    <Stat stat=user.following_cnt info="Loving"/>
                    <Stat stat=user.hots info="Hots"/>
                    <Stat stat=user.nots info="Nots"/>
                </div>
                <ListSwitcher user_canister/>
            </div>
        </div>
    }
}

#[component]
pub fn ProfileView() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let principal = move || {
        params.with(|p| {
            let ProfileParams { id } = p.as_ref().ok()?;

            Principal::from_text(id).ok()
        })
    };

    let user_details = create_resource(principal, |principal| async move {
        let canisters = unauth_canisters();
        let user_canister = canisters
            .get_individual_canister_by_user_principal(principal?)
            .await
            .ok()??;
        let user = canisters.individual_user(user_canister);
        let user_details = user.get_profile_details().await.ok()?;
        Some((user_details.into(), user_canister))
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                user_details
                    .get()
                    .map(|user| {
                        view! {
                            {move || {
                                if let Some((user, user_canister)) = user.clone() {
                                    view! { <ProfileViewInner user user_canister/> }
                                } else {
                                    view! { <Redirect path="/"/> }
                                }
                            }}
                        }
                    })
            }}

        </Suspense>
    }
}
