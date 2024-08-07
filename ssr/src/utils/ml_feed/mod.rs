use crate::consts::ML_FEED_GRPC_URL;
use candid::Principal;

use super::types::PostId;

#[cfg(feature = "hydrate")]
pub mod ml_feed_grpcweb {
    use super::*;
    use crate::utils::ml_feed::ml_feed_grpcweb::ml_feed_proto::{
        ml_feed_client::MlFeedClient, FeedRequest, PostItem,
    };
    use crate::utils::posts::PostDetails;
    use tonic_web_wasm_client::Client;

    pub mod ml_feed_proto {
        include!(concat!(env!("OUT_DIR"), "/grpc-web/ml_feed.rs"));
    }

    #[derive(Clone)]
    pub struct MLFeed {
        pub client: MlFeedClient<Client>,
    }

    impl Default for MLFeed {
        fn default() -> Self {
            let client = Client::new(ML_FEED_GRPC_URL.to_string());

            Self {
                client: MlFeedClient::new(client),
            }
        }
    }

    impl MLFeed {
        pub async fn get_next_feed(
            mut self,
            canister_id: &Principal,
            limit: u32,
            filter_list: Vec<PostDetails>,
        ) -> Result<Vec<PostId>, tonic_2::Status> {
            let request = FeedRequest {
                canister_id: canister_id.to_string(),
                filter_posts: filter_list
                    .iter()
                    .map(|item| PostItem {
                        post_id: item.post_id as u32,
                        canister_id: item.canister_id.to_string(),
                        video_id: item.uid.clone(),
                    })
                    .collect(),
                num_results: limit,
            };

            let response = self.client.get_feed(request).await.map_err(|e| {
                tonic_2::Status::new(
                    tonic_2::Code::Internal,
                    format!("Error fetching posts: {:?}", e),
                )
            })?;

            let feed_res = response.into_inner().feed;

            Ok(feed_res
                .iter()
                .map(|item| {
                    (
                        Principal::from_text(&item.canister_id).unwrap(),
                        item.post_id as u64,
                    )
                })
                .collect())
        }
    }
}

#[cfg(feature = "ssr")]
pub mod ml_feed_grpc {
    use super::*;
    use crate::utils::ml_feed::ml_feed_grpc::ml_feed_proto::{
        ml_feed_client::MlFeedClient, FeedRequest, PostItem,
    };
    use crate::utils::posts::PostDetails;
    use tonic::transport::Channel;

    pub mod ml_feed_proto {
        tonic::include_proto!("ml_feed");
    }

    pub async fn get_start_feed(
        canister_id: &Principal,
        limit: u32,
        filter_list: Vec<PostDetails>,
    ) -> Result<Vec<PostId>, tonic::Status> {
        let channel = Channel::from_static("https://yral-ml-feed-server-staging.fly.dev:443")
            .connect()
            .await
            .expect("Couldn't connect to ML feed server");

        // let mut client = MlFeedClient::connect(ML_FEED_GRPC_URL).await.unwrap();

        let mut client = MlFeedClient::new(channel);

        let request = tonic::Request::new(FeedRequest {
            canister_id: canister_id.to_string(),
            filter_posts: filter_list
                .iter()
                .map(|item| PostItem {
                    post_id: item.post_id as u32,
                    canister_id: item.canister_id.to_string(),
                    video_id: item.uid.clone(),
                })
                .collect(),
            num_results: limit,
        });

        let response = client.get_feed(request).await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Internal,
                format!("error fetching posts: {:?}", e),
            )
        })?;

        let feed_res = response.into_inner().feed;

        Ok(feed_res
            .iter()
            .map(|item| {
                (
                    Principal::from_text(&item.canister_id).unwrap(),
                    item.post_id as u64,
                )
            })
            .collect())
    }
}

// TODO: remove
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
