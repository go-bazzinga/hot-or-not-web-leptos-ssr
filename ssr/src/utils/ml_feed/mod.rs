use crate::consts::ML_FEED_GRPC_URL;
use candid::Principal;
use leptos::RwSignal;
use tonic_web_wasm_client::Client;

use super::types::PostId;

use crate::utils::ml_feed::ml_feed_proto::{ml_feed_client::MlFeedClient, FeedRequest, PostItem};
use leptos::*;

pub mod ml_feed_proto {
    tonic_2::include_proto!("ml_feed");
}

#[derive(Clone)]
pub struct MLFeed {
    pub client: MlFeedClient<Client>,
}

impl Default for MLFeed {
    fn default() -> Self {
        let ml_feed_url = "http://localhost:50051".to_string();
        let client = Client::new(ml_feed_url);

        Self { client: MlFeedClient::new(client) }
    }
}

// #[cfg(not(feature = "local-feed"))]
pub mod ml_feed_impl {
    use super::*;

    pub async fn get_next_feed(
        canister_id: &Principal,
        limit: u32,
        filter_list: Vec<PostId>,
    ) -> Result<Vec<PostId>, tonic_2::Status> {

        // let mut ml_feed = MLFeed::default();

        let mut ml_feed: MLFeed = expect_context();

        let request = FeedRequest {
            canister_id: canister_id.to_string(),
            filter_posts: vec![], // filter_list.iter().map(|item| PostItem {post_id: item.1 as u32, canister_id: item.0.to_string()}).collect(),
            num_results: limit,
        };

        let response =  ml_feed.client.get_feed(request).await.map_err(|e| {
            leptos::logging::log!("error fetching posts: {:?}", e);
            tonic_2::Status::new(tonic_2::Code::Internal, "error fetching posts")
        })?;

        let feed_res = response.into_inner().feed;

        Ok(feed_res.iter().map(|item| (Principal::from_text(&item.canister_id).unwrap(), item.post_id as u64)).collect())
    }
}

// #[cfg(feature = "local-feed")]
// pub mod local_feed_impl {
//     use super::*;

//     pub async fn get_next_feed() -> Result<Vec<PostId>, tonic_2::Status> {
//         let posts = vec![
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 125,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 124,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 123,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 122,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 121,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 120,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 119,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 118,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 117,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 116,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 115,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 114,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 113,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 112,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 111,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 110,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 109,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 108,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 107,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 106,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 105,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 104,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 103,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 102,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 101,
//             ),
//             (
//                 Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
//                 100,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 26,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 25,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 24,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 23,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 22,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 21,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 20,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 19,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 18,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 17,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 16,
//             ),
//             (
//                 Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(),
//                 15,
//             ),
//         ];

//         Ok(posts)
//     }
// }
