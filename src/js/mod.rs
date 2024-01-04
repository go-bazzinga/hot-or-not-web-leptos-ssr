pub mod wasp {
    use std::ops::Deref;

    use leptos::{html::Video, HtmlElement};
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/js/wasp-wrapper.js")]
    extern "C" {
        pub type WaspHlsPlayer;

        fn buildPlayer(videoElement: JsValue, config: JsValue) -> WaspHlsPlayer;

        #[wasm_bindgen(method)]
        pub fn load(this: &WaspHlsPlayer, url: &str);
    }

    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct WaspHlsConfig {
        pub buffer_goal: Option<f64>,
        pub segment_max_retry: Option<f64>,
        pub segment_request_timeout: Option<f64>,
        pub segment_backoff_base: Option<f64>,
        pub segment_backoff_max: Option<f64>,
        pub multi_variant_playlist_max_retry: Option<f64>,
        pub multi_variant_playlist_request_timeout: Option<f64>,
        pub multi_variant_playlist_backoff_base: Option<f64>,
        pub multi_variant_playlist_backoff_max: Option<f64>,
        pub media_playlist_max_retry: Option<f64>,
        pub media_playlist_request_timeout: Option<f64>,
        pub media_playlist_backoff_base: Option<f64>,
        pub media_playlist_backoff_max: Option<f64>,
    }

    impl WaspHlsPlayer {
        pub fn new(video_element: &HtmlElement<Video>, config: Option<WaspHlsConfig>) -> Self {
            let video_raw: &JsValue = video_element.deref();
            let conf = serde_wasm_bindgen::to_value(&config).unwrap();
            buildPlayer(video_raw.clone(), conf)
        }
    }
}
