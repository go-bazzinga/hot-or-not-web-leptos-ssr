use std::pin::Pin;

use candid::Principal;
use futures::{stream::FuturesOrdered, Stream, StreamExt};
use serde::{Deserialize, Serialize};

use crate::{
    canister::{
        individual_user_template::PostDetailsForFrontend,
        post_cache::{self, NsfwFilter},
    },
    state::canisters::Canisters,
    utils::profile::propic_from_principal,
};

use super::error::PostViewError;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FetchCursor {
    pub start: u64,
    pub limit: u64,
}

impl Default for FetchCursor {
    fn default() -> Self {
        Self {
            start: 0,
            limit: 10,
        }
    }
}

impl FetchCursor {
    pub fn advance(&mut self) {
        self.start += self.limit;
        self.limit = 20;
    }
}

pub async fn get_post_uid<const AUTH: bool>(
    canisters: &Canisters<AUTH>,
    user_canister: Principal,
    post_id: u64,
) -> Result<Option<PostDetails>, PostViewError> {
    let post_creator_can = canisters.individual_user(user_canister);
    let post_details = match post_creator_can
        .get_individual_post_details_by_id(post_id)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            log::warn!("failed to get post details: {}, skipping", e);
            return Ok(None);
        }
    };

    let post_uuid = &post_details.video_uid;
    let req_url = format!(
        "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/manifest/video.m3u8",
        post_uuid,
    );
    let res = reqwest::Client::default().head(req_url).send().await?;
    if res.status() != 200 {
        return Ok(None);
    }

    Ok(Some(PostDetails::from_canister_post(
        AUTH,
        user_canister,
        post_details,
    )))
}

pub async fn post_liked_by_me(
    canisters: &Canisters<true>,
    post_canister: Principal,
    post_id: u64,
) -> Result<bool, PostViewError> {
    let individual = canisters.individual_user(post_canister);
    let post = individual
        .get_individual_post_details_by_id(post_id)
        .await?;
    Ok(post.liked_by_me)
}

#[derive(Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PostDetails {
    pub canister_id: Principal,
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

pub struct VideoFetchStream<'a, const AUTH: bool> {
    canisters: &'a Canisters<AUTH>,
    cursor: FetchCursor,
}

impl<'a, const AUTH: bool> VideoFetchStream<'a, AUTH> {
    pub fn new(canisters: &'a Canisters<AUTH>, cursor: FetchCursor) -> Self {
        Self { canisters, cursor }
    }

    pub async fn fetch_post_uids_chunked(
        self,
        chunks: usize,
        allow_nsfw: bool,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Vec<Result<PostDetails, PostViewError>>> + 'a>>,
        PostViewError,
    > {
        let post_cache = self.canisters.post_cache();
        let top_posts_fut = post_cache
            .get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed_cursor(
                self.cursor.start,
                self.cursor.limit,
                None,
                None,
                Some(if allow_nsfw {
                    NsfwFilter::IncludeNsfw
                } else {
                    NsfwFilter::ExcludeNsfw
                }),
            );
        // TODO: error handling
        let post_cache::Result_::Ok(top_posts) = top_posts_fut.await? else {
            return Err(PostViewError::Canister(
                "canister refused to send posts".into(),
            ));
        };
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| get_post_uid(self.canisters, item.publisher_canister_id, item.post_id))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(Box::pin(chunk_stream))
    }
}
