use std::pin::Pin;

use candid::Principal;
use futures::{stream::FuturesOrdered, Stream, StreamExt};

use crate::{
    canister::post_cache::{self, NsfwFilter},
    state::canisters::Canisters,
    utils::posts::{get_post_uid, FetchCursor, PostDetails, PostViewError},
};

pub async fn post_liked_by_me(
    canisters: &Canisters<true>,
    post_canister: Principal,
    post_id: u64,
) -> Result<(bool, u64), PostViewError> {
    let individual = canisters.individual_user(post_canister).await?;
    let post = individual
        .get_individual_post_details_by_id(post_id)
        .await?;
    Ok((post.liked_by_me, post.like_count))
}

type PostsStream<'a> = Pin<Box<dyn Stream<Item = Vec<Result<PostDetails, PostViewError>>> + 'a>>;

pub struct FetchVideosRes<'a> {
    pub posts_stream: PostsStream<'a>,
    pub end: bool,
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
    ) -> Result<FetchVideosRes<'a>, PostViewError> {
        let post_cache = self.canisters.post_cache().await?;
        let top_posts_fut = post_cache
            .get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed_cursor(
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
        let top_posts = match top_posts_fut.await? {
            post_cache::Result_::Ok(top_posts) => top_posts,
            post_cache::Result_::Err(post_cache::TopPostsFetchError::ReachedEndOfItemsList) => {
                return Ok(FetchVideosRes {
                    posts_stream: Box::pin(futures::stream::empty()),
                    end: true,
                })
            }
            post_cache::Result_::Err(_) => {
                return Err(PostViewError::Canister(
                    "canister refused to send posts".into(),
                ))
            }
        };

        let end = top_posts.len() < self.cursor.limit as usize;
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| get_post_uid(self.canisters, item.publisher_canister_id, item.post_id))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(FetchVideosRes {
            posts_stream: Box::pin(chunk_stream),
            end,
        })
    }
}
