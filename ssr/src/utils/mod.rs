use serde::{Deserialize, Serialize};
use web_time::{Duration, SystemTime};

pub mod event_streaming;
pub mod ic;
pub mod icon;
pub mod ml_feed;
pub mod posts;
pub mod profile;
pub mod report;
pub mod route;
pub mod timestamp;
pub mod types;
pub mod user;
pub mod web;

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

#[cfg(not(clippy))]
#[cfg(all(feature = "ga4", feature = "ssr"))]
pub mod off_chain {
    tonic::include_proto!("off_chain");
}
