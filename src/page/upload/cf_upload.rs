use candid::Principal;
#[cfg(all(feature = "cloudflare", feature = "ssr"))]
use cf_impl::server_func::*;
#[cfg(feature = "cloudflare")]
pub use cf_impl::{publish_video, upload_video_stream};
use leptos::*;
#[cfg(all(not(feature = "cloudflare"), feature = "ssr"))]
use mock_impl::server_func::*;
#[cfg(not(feature = "cloudflare"))]
pub use mock_impl::{publish_video, upload_video_stream};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UploadInfo {
    pub uid: String,
    // pub creator_id: String,
    pub upload_url: String,
}

#[server(GetUploadInfo)]
pub async fn get_upload_info(
    creator: Principal,
    hashtags: Vec<String>,
    description: String,
    file_name: String,
) -> Result<UploadInfo, ServerFnError> {
    // TODO(SECURITY): authenticate creator

    if description.len() < 10 {
        return Err(ServerFnError::Args(
            "Description must be at least 10 characters".into(),
        ));
    }
    if hashtags.len() > 8 {
        return Err(ServerFnError::Args("Too many hashtags".into()));
    }

    get_upload_info_impl(creator, hashtags, description, file_name).await
}

#[server(GetVideoStatus)]
pub async fn get_video_status(uid: String) -> Result<String, ServerFnError> {
    get_video_status_impl(uid).await
}

#[server(EditVideoMeta)]
pub async fn edit_video_meta(uid: String, post_id: u64) -> Result<String, ServerFnError> {
    edit_video_meta_impl(uid, post_id).await
}

#[cfg(feature = "cloudflare")]
mod cf_impl {
    use leptos::ServerFnError;

    use crate::{
        canister::individual_user_template::{PostDetailsFromFrontend, Result_},
        state::canisters::Canisters,
    };

    use super::UploadInfo;

    #[cfg(feature = "ssr")]
    pub mod server_func {
        use candid::Principal;
        use gob_cloudflare::{
            api::stream_videos::{CreateDownloads, DirectUpload, EditVideoDetails, VideoDetails},
            CloudflareAuth,
        };
        use leptos::{expect_context, server_fn::ServerFn, ServerFnError};

        use crate::consts::CF_WATERMARK_UID;

        use super::UploadInfo;
        use std::time::Duration;

        pub async fn edit_video_meta_impl(
            uid: String,
            post_id: u64,
        ) -> Result<String, ServerFnError> {
            let cf_api: CloudflareAuth = expect_context();
            // get the existing meta from video
            let req = VideoDetails::new(uid.clone());
            let res = cf_api.send_auth(req).await?;
            let meta = res.meta;

            // add new meta to the video
            let req = EditVideoDetails::new(uid, meta).add_meta("post_id", post_id.to_string());
            let res = cf_api.send_auth(req).await?;

            Ok(res.status.state)
        }

        pub async fn get_upload_info_impl(
            creator: Principal,
            hashtags: Vec<String>,
            description: String,
            file_name: String,
        ) -> Result<UploadInfo, ServerFnError> {
            let cf_api: CloudflareAuth = expect_context();
            let req = DirectUpload::default()
                .creator(creator.to_text())
                .add_meta("hashtags", hashtags.join(","))
                .add_meta("description", description)
                .add_meta("fileName", file_name)
                .add_meta("uploadType", "challenge")
                .add_meta("creator", creator.to_text())
                .watermark(CF_WATERMARK_UID)
                .max_duration(Duration::from_secs(60));
            let res = cf_api.send_auth(req).await?;

            Ok(UploadInfo {
                uid: res.uid,
                upload_url: res.upload_url,
                // creator_id: creator.to_text()
            })
        }

        pub async fn get_video_status_impl(uid: String) -> Result<String, ServerFnError> {
            let cf_api: CloudflareAuth = expect_context();
            let req = VideoDetails::new(uid.clone());
            let res = cf_api.send_auth(req).await?;
            let state = res.status.state;
            if state != "ready" {
                return Ok(state);
            }
            let req = CreateDownloads::new(uid);
            _ = cf_api.send_auth(req).await?;

            Ok(state)
        }
    }

    pub async fn upload_video_stream(
        _upload_res: &UploadInfo,
        _file: &gloo::file::File,
    ) -> Result<(), gloo::net::Error> {
        #[cfg(feature = "hydrate")]
        {
            use gloo::net::http::Request;
            use leptos::web_sys::FormData;
            let form = FormData::new().unwrap();
            form.append_with_blob("file", _file.as_ref()).unwrap();
            let req = Request::post(&_upload_res.upload_url).body(form).unwrap();
            req.send().await?;
        }
        Ok(())
    }

    pub async fn publish_video(
        canisters: Canisters<true>,
        hashtags: Vec<String>,
        description: String,
        uid: String,
        enable_hot_or_not: bool,
        is_nsfw: bool,
    ) -> Result<u64, ServerFnError> {
        let user = canisters.authenticated_user();
        let res = user
            .add_post_v_2(PostDetailsFromFrontend {
                hashtags,
                description,
                video_uid: uid,
                creator_consent_for_inclusion_in_hot_or_not: enable_hot_or_not,
                is_nsfw,
            })
            .await?;
        match res {
            Result_::Ok(p) => Ok(p),
            Result_::Err(e) => return Err(ServerFnError::new(e)),
        }
        // user.update_post_as_ready_to_view(post_id).await?;
    }
}

#[cfg(not(feature = "cloudflare"))]
mod mock_impl {
    use super::UploadInfo;
    use crate::state::canisters::Canisters;
    use leptos::ServerFnError;

    #[cfg(feature = "ssr")]
    pub mod server_func {
        use candid::Principal;
        use leptos::ServerFnError;
        use std::time::Duration;

        use super::UploadInfo;

        pub async fn get_upload_info_impl(
            _creator: Principal,
            _hashtags: Vec<String>,
            _description: String,
            _file_name: String,
        ) -> Result<UploadInfo, ServerFnError> {
            Ok(UploadInfo {
                uid: "mock".into(),
                upload_url: "http://mock.com".into(),
            })
        }

        pub async fn get_video_status_impl(_uid: String) -> Result<String, ServerFnError> {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok("ready".into())
        }

        pub async fn edit_video_meta_impl(
            _uid: String,
            _post_id: u64,
        ) -> Result<String, ServerFnError> {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok("processing".into())
        }
    }

    pub async fn upload_video_stream(
        _upload_res: &UploadInfo,
        _file: &gloo::file::File,
    ) -> Result<(), gloo::net::Error> {
        use gloo::timers::future::TimeoutFuture;
        TimeoutFuture::new(1000).await;
        Ok(())
    }

    pub async fn publish_video(
        _canisters: Canisters<true>,
        _hashtags: Vec<String>,
        _description: String,
        _uid: String,
        _enable_hot_or_not: bool,
        _is_nsfw: bool,
    ) -> Result<(), ServerFnError> {
        use gloo::timers::future::TimeoutFuture;
        TimeoutFuture::new(1000).await;
        Ok(())
    }
}
