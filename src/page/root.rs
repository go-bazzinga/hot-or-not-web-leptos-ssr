use crate::component::spinner::Spinner;
use candid::Principal;
use leptos::*;
use leptos_router::use_navigate;

#[cfg(feature = "ssr")]
use crate::{canister::post_cache, state::canisters::Canisters};

#[server]
async fn get_top_post_id() -> Result<Option<(Principal, u64)>, ServerFnError> {
    let canisters = expect_context::<Canisters>();
    let post_cache = canisters.post_cache();

    let top_items = match post_cache
        .get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed(0, 1)
        .await?
    {
        post_cache::Result_::Ok(items) => items,
        post_cache::Result_::Err(_) => {
            return Err(ServerFnError::ServerError(format!(
                "failed to fetch top post"
            )));
        }
    };
    let Some(top_item) = top_items.first() else {
        return Ok(None);
    };

    Ok(Some((top_item.publisher_canister_id, top_item.post_id)))
}

#[component]
pub fn RootPage() -> impl IntoView {
    let target_url = create_resource(|| (), |_| get_top_post_id());

    view! {
        <Suspense fallback=|| {
            view! {
                <div class="h-screen w-screen grid grid-cols-1 bg-slate-950 justify-items-center place-content-center">
                    <Spinner/>
                </div>
            }
        }>
            {move || {
                let url = match target_url.get() {
                    Some(Ok(Some((canister, post_id)))) => {
                        format!("/hot-or-not/{canister}/{post_id}")
                    }
                    Some(Ok(None)) => "/error?err=No Posts Found".to_string(),
                    Some(Err(e)) => format!("/error?err={e}"),
                    None => return,
                };
                let nav = use_navigate();
                nav(&url, Default::default());
            }}

        </Suspense>
    }
}
