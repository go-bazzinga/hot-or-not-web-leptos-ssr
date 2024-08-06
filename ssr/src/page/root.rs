use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::component::spinner::FullScreenSpinner;
#[cfg(feature = "ssr")]
use crate::{canister::post_cache, state::canisters::unauth_canisters};

#[server]
async fn get_top_post_id() -> Result<Option<(Principal, u64)>, ServerFnError> {
    let canisters = unauth_canisters();
    let post_cache = canisters.post_cache().await?;

    let top_items = match post_cache
        .get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed_cursor(
            0,
            1,
            None,
            None,
            Some(post_cache::NsfwFilter::ExcludeNsfw),
        )
        .await?
    {
        post_cache::Result_::Ok(items) => items,
        post_cache::Result_::Err(_) => {
            return Err(ServerFnError::ServerError(
                "failed to fetch top post".to_string(),
            ));
        }
    };
    let Some(top_item) = top_items.first() else {
        return Ok(None);
    };

    Ok(Some((top_item.publisher_canister_id, top_item.post_id)))
}


#[server]
async fn get_top_post_id_mlfeed() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use crate::utils::ml_feed::ml_feed_grpc::get_start_feed;

    let canisters = unauth_canisters();
    let user_canister_principal = canisters.user_canister();
    let top_posts_fut = get_start_feed(&user_canister_principal, 1, vec![]);

    let top_items = match top_posts_fut.await
    {
        Ok(top_posts) => top_posts,
        Err(e) => {
            return Err(ServerFnError::ServerError(
                "failed to fetch top post ml feed".to_string(),
            ));
        }
    };
    let Some(top_item) = top_items.first() else {
        return Ok(None);
    };

    Ok(Some((top_item.0, top_item.1)))
}


#[component]
pub fn RootPage() -> impl IntoView {
    let target_post = create_resource(|| (), |_| get_top_post_id_mlfeed());

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                target_post
                    .get()
                    .map(|u| {
                        let url = match u {
                            Ok(Some((canister, post_id))) => {
                                format!("/hot-or-not/{canister}/{post_id}")
                            }
                            Ok(None) => "/error?err=No Posts Found".to_string(),
                            Err(e) => format!("/error?err={e}"),
                        };
                        view! { <Redirect path=url /> }
                    })
            }}

        </Suspense>
    }
}
