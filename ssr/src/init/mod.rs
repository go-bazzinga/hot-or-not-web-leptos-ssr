#[cfg(feature = "local-bin")]
pub mod containers;

use std::env;

use axum_extra::extract::cookie::Key;
use leptos::LeptosOptions;
use leptos_router::RouteListing;

use crate::{
    auth::server_impl::store::KVStoreImpl,
    state::{canisters::Canisters, server::AppState},
};

#[cfg(feature = "cloudflare")]
fn init_cf() -> gob_cloudflare::CloudflareAuth {
    use gob_cloudflare::{CloudflareAuth, Credentials};
    let creds = Credentials {
        token: env::var("CF_TOKEN").expect("`CF_TOKEN` is required!"),
        account_id: env::var("CF_ACCOUNT_ID").expect("`CF_ACCOUNT_ID` is required!"),
    };
    CloudflareAuth::new(creds)
}

fn init_cookie_key() -> Key {
    let cookie_key_str = env::var("COOKIE_KEY").expect("`COOKIE_KEY` is required!");
    let cookie_key_raw =
        hex::decode(cookie_key_str).expect("Invalid `COOKIE_KEY` (must be length 128 hex)");
    Key::from(&cookie_key_raw)
}

#[cfg(feature = "oauth-ssr")]
fn init_google_oauth() -> openidconnect::core::CoreClient {
    use crate::consts::google::{GOOGLE_AUTH_URL, GOOGLE_ISSUER_URL, GOOGLE_TOKEN_URL};
    use openidconnect::{
        core::CoreClient, AuthUrl, ClientId, ClientSecret, IssuerUrl, RedirectUrl, TokenUrl,
    };

    let client_id = env::var("GOOGLE_CLIENT_ID").expect("`GOOGLE_CLIENT_ID` is required!");
    let client_secret =
        env::var("GOOGLE_CLIENT_SECRET").expect("`GOOGLE_CLIENT_SECRET` is required!");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URL").expect("`GOOGLE_REDIRECT_URL` is required!");

    CoreClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        IssuerUrl::new(GOOGLE_ISSUER_URL.to_string()).unwrap(),
        AuthUrl::new(GOOGLE_AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).unwrap()),
        None,
        // We don't validate id_tokens against Google's public keys
        Default::default(),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap())
}

#[cfg(feature = "ga4")]
async fn init_grpc_offchain_channel() -> tonic::transport::Channel {
    use crate::consts::OFF_CHAIN_AGENT_GRPC_URL;
    use tonic::transport::Channel;

    let off_chain_agent_url = OFF_CHAIN_AGENT_GRPC_URL.as_ref();
    Channel::from_static(off_chain_agent_url)
        .connect()
        .await
        .expect("Couldn't connect to off-chain agent")
}



#[cfg(feature = "backend-admin")]
fn init_admin_canisters() -> crate::state::admin_canisters::AdminCanisters {
    use crate::state::admin_canisters::AdminCanisters;

    #[cfg(feature = "local-bin")]
    {
        use ic_agent::identity::Secp256k1Identity;
        use k256::SecretKey;
        use yral_testcontainers::backend::ADMIN_SECP_BYTES;

        let sk = SecretKey::from_bytes(&ADMIN_SECP_BYTES.into()).unwrap();
        let identity = Secp256k1Identity::from_private_key(sk);
        AdminCanisters::new(identity)
    }

    #[cfg(not(feature = "local-bin"))]
    {
        use ic_agent::identity::BasicIdentity;

        let admin_id_pem =
            env::var("BACKEND_ADMIN_IDENTITY").expect("`BACKEND_ADMIN_IDENTITY` is required!");
        let admin_id_pem_by = admin_id_pem.as_bytes();
        let admin_id =
            BasicIdentity::from_pem(admin_id_pem_by).expect("Invalid `BACKEND_ADMIN_IDENTITY`");
        AdminCanisters::new(admin_id)
    }
}

pub struct AppStateRes {
    pub app_state: AppState,
    #[cfg(feature = "local-bin")]
    pub containers: containers::TestContainers,
}

pub struct AppStateBuilder {
    leptos_options: LeptosOptions,
    routes: Vec<RouteListing>,
    #[cfg(feature = "local-bin")]
    containers: containers::TestContainers,
}

impl AppStateBuilder {
    pub fn new(leptos_options: LeptosOptions, routes: Vec<RouteListing>) -> Self {
        Self {
            leptos_options,
            routes,
            #[cfg(feature = "local-bin")]
            containers: containers::TestContainers::default(),
        }
    }

    async fn init_kv(&mut self) -> KVStoreImpl {
        #[cfg(feature = "redis-kv")]
        {
            use crate::auth::server_impl::store::redis_kv::RedisKV;
            let redis_url: String;
            #[cfg(feature = "local-bin")]
            {
                self.containers.start_redis().await;
                redis_url = "redis://127.0.0.1:6379".to_string();
            }
            #[cfg(not(feature = "local-bin"))]
            {
                redis_url = env::var("REDIS_URL").expect("`REDIS_URL` is required!");
            }
            KVStoreImpl::Redis(RedisKV::new(&redis_url).await.unwrap())
        }

        #[cfg(not(feature = "redis-kv"))]
        {
            use crate::auth::server_impl::store::redb_kv::ReDBKV;
            KVStoreImpl::ReDB(ReDBKV::new().expect("Failed to initialize ReDB"))
        }
    }

    pub async fn build(mut self) -> AppStateRes {
        let kv = self.init_kv().await;
        #[cfg(feature = "local-bin")]
        {
            self.containers.start_backend().await;
            self.containers.start_metadata().await;
        }

        let app_state = AppState {
            leptos_options: self.leptos_options,
            canisters: Canisters::default(),
            routes: self.routes,
            #[cfg(feature = "backend-admin")]
            admin_canisters: init_admin_canisters(),
            #[cfg(feature = "cloudflare")]
            cloudflare: init_cf(),
            kv,
            cookie_key: init_cookie_key(),
            #[cfg(feature = "oauth-ssr")]
            google_oauth: init_google_oauth(),
            #[cfg(feature = "ga4")]
            grpc_offchain_channel: init_grpc_offchain_channel().await,
        };

        AppStateRes {
            app_state,
            #[cfg(feature = "local-bin")]
            containers: self.containers,
        }
    }
}
