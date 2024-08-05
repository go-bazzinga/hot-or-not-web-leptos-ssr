use serde::{Deserialize, Serialize};
use web_time::{Duration, SystemTime};

pub mod event_streaming;
pub mod ic;
pub mod icon;
pub mod posts;
pub mod profile;
pub mod report;
pub mod route;
pub mod timestamp;
pub mod types;
pub mod user;
pub mod web;

#[cfg(feature = "hydrate")]
pub mod ml_feed;

pub fn current_epoch() -> Duration {
    web_time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}

/// Wrapper for PartialEq that always returns false
/// this is currently only used for resources
/// this does not provide a sane implementation of PartialEq
#[derive(Clone, Serialize, Deserialize)]
pub struct MockPartialEq<T>(pub T);

impl<T> PartialEq for MockPartialEq<T> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[cfg(all(feature = "ga4", feature = "ssr"))]
pub mod off_chain {
    tonic::include_proto!("off_chain");
}


// TODO: to be removed
pub mod local_feed_impl {
    use candid::Principal;
    use super::types::PostId;

    pub async fn get_next_feed() -> Result<Vec<PostId>, String> {
        let posts = vec![
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                125,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                124,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                123,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                122,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                121,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                120,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                119,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                118,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                117,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                116,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                115,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                114,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                113,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                112,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                111,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                110,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                109,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                108,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                107,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                106,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                105,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                104,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                103,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                102,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                101,
            ),
            (
                Principal::from_text("76qol-iiaaa-aaaak-qelkq-cai").unwrap(),
                100,
            ),
        ];

        Ok(posts)
    }
}
