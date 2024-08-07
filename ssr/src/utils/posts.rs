use candid::Principal;
use serde::{Deserialize, Serialize};

use crate::{
    canister::individual_user_template::PostDetailsForFrontend, state::canisters::Canisters,
};

use super::profile::propic_from_principal;

use ic_agent::AgentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostViewError {
    #[error("IC agent error {0}")]
    Agent(#[from] AgentError),
    #[error("Canister error {0}")]
    Canister(String),
    #[error("http fetch error {0}")]
    HttpFetch(#[from] reqwest::Error),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FetchCursor {
    pub start: u64,
    pub limit: u64,
}

impl Default for FetchCursor {
    fn default() -> Self {
        Self {
            start: 0,
            limit: 25,
        }
    }
}

impl FetchCursor {
    pub fn advance(&mut self) {
        self.start += self.limit;
        self.limit = 25;
    }
}

#[derive(Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PostDetails {
    pub canister_id: Principal, // canister id of the publishing canister.
    pub post_id: u64,
    pub uid: String,
    pub description: String,
    pub views: u64,
    pub likes: u64,
    pub display_name: String,
    pub propic_url: String,
    /// Whether post is liked by the authenticated
    /// user or not, None if unknown
    pub liked_by_user: Option<bool>,
    pub poster_principal: Principal,
    pub hastags: Vec<String>,
    pub is_nsfw: bool,
    pub hot_or_not_feed_ranking_score: Option<u64>,
}

impl PostDetails {
    pub fn from_canister_post(
        authenticated: bool,
        canister_id: Principal,
        details: PostDetailsForFrontend,
    ) -> Self {
        Self {
            canister_id,
            post_id: details.id,
            uid: details.video_uid,
            description: details.description,
            views: details.total_view_count,
            likes: details.like_count,
            display_name: details
                .created_by_display_name
                .or(details.created_by_unique_user_name)
                .unwrap_or_else(|| details.created_by_user_principal_id.to_text()),
            propic_url: details
                .created_by_profile_photo_url
                .unwrap_or_else(|| propic_from_principal(details.created_by_user_principal_id)),
            liked_by_user: authenticated.then_some(details.liked_by_me),
            poster_principal: details.created_by_user_principal_id,
            hastags: details.hashtags,
            is_nsfw: details.is_nsfw,
            hot_or_not_feed_ranking_score: details.hot_or_not_feed_ranking_score,
        }
    }
}

pub async fn get_post_uid<const AUTH: bool>(
    canisters: &Canisters<AUTH>,
    user_canister: Principal,
    post_id: u64,
) -> Result<Option<PostDetails>, PostViewError> {
    let post_creator_can = canisters.individual_user(user_canister).await?;
    let post_details = match post_creator_can
        .get_individual_post_details_by_id(post_id)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            log::warn!(
                "failed to get post details for {} {}: {}, skipping",
                user_canister.to_string(),
                post_id,
                e
            );
            return Ok(None);
        }
    };

    // TODO: Add this filter in new method
    // if post_details.status == PostStatus::BannedDueToUserReporting {
    //     return Ok(None);
    // }

    let post_uuid = &post_details.video_uid;
    let req_url = format!(
        "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/manifest/video.m3u8",
        post_uuid,
    );
    let res = reqwest::Client::default().head(req_url).send().await;
    if res.is_err() || (res.is_ok() && res.unwrap().status() != 200) {
        return Ok(None);
    }

    Ok(Some(PostDetails::from_canister_post(
        AUTH,
        user_canister,
        post_details,
    )))
}
