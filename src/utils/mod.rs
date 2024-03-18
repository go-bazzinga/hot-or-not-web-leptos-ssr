use web_time::{Duration, SystemTime};

pub mod event_streaming;
pub mod icon;
pub mod profile;
pub mod route;
pub mod timestamp;
pub mod web;

pub fn current_epoch() -> Duration {
    web_time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}

/// Wrapper for PartialEq that always returns false
/// this is currently only used for resources
/// this does not provide a sane implementation of PartialEq
#[derive(Clone)]
pub struct MockPartialEq<T>(pub T);

impl<T> PartialEq for MockPartialEq<T> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
