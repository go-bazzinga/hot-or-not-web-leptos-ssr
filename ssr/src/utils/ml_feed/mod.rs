use candid::Principal;
use leptos::RwSignal;
use tonic::transport::Channel;
use crate::consts::ML_FEED_GRPC_URL;

use super::types::PostId;

#[derive(Clone, Default)]
pub struct MLFeed {
    pub channel: Option<Channel>,
}


pub async fn init_mlfeed_grpc_channel() -> Channel {
    let ml_feed_url = ML_FEED_GRPC_URL.as_ref();
    Channel::from_static(ml_feed_url)
        .connect()
        .await
        .expect("Couldn't connect to ML feed server")
}

impl MLFeed {
    pub async fn get_channel(&mut self) -> &Channel {
        if self.channel.is_none() {
            self.channel = Some(init_mlfeed_grpc_channel().await);
        }
        self.channel.as_ref().unwrap()
    }
}


#[cfg(not(feature = "local-feed"))]
pub mod ml_feed_impl {
    use super::*;

    pub async fn get_next_feed(
        ml_feed_channel: MLFeed,
        canister_id: &Principal,
        limit: u32,
        filter_list: Vec<PostId>,
    ) -> Result<Vec<PostId>, tonic::Status> {

        // let mut client = MLFeedServiceClient::new(self.channel.clone());
        // let request = tonic::Request::new(MLFeedRequest {
        //     user_id: user_id.to_string(),
        //     limit: limit as u32,
        // });
        // let response = client.get_recommendations(request).await?;
        // Ok(response.into_inner().recommendations)

        Ok(vec![])
    }
}

#[cfg(feature = "local-feed")]
pub mod local_feed_impl {
    use super::*;

    pub async fn get_next_feed() -> Result<Vec<PostId>, tonic::Status> {

        let posts = vec![
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 125),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 124),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 123),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 122),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 121),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 120),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 119),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 118),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 117),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 116),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 115),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 114),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 113),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 112),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 111),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 110),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 109),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 108),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 107),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 106),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 105),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 104),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 103),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 102),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 101),
            (Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(), 100),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 26),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 25),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 24),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 23),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 22),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 21),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 20),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 19),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 18),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 17),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 16),
            (Principal::from_text("vcqbz-sqaaa-aaaag-aesbq-cai").unwrap(), 15),
        ];

        Ok(posts)
    }
}

