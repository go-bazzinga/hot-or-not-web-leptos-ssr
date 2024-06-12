#[cfg(feature = "oauth-ssr")]
pub mod google;
pub mod store;

use axum::response::IntoResponse;
use axum_extra::extract::{
    cookie::{Cookie, Key, SameSite},
    SignedCookieJar,
};
use candid::Principal;
use http::header;
use ic_agent::{
    identity::{Delegation, Secp256k1Identity, SignedDelegation},
    Identity,
};
use leptos::{expect_context, ServerFnError};
use leptos_axum::{extract_with_state, ResponseOptions};
use rand_chacha::rand_core::OsRng;

use crate::{
    consts::auth::{DELEGATION_MAX_AGE, REFRESH_MAX_AGE, REFRESH_TOKEN_COOKIE},
    utils::current_epoch,
};

use self::store::{KVStore, KVStoreImpl};

use super::{DelegatedIdentityWire, RefreshToken};

impl DelegatedIdentityWire {
    pub fn delegate(from: &impl Identity) -> Self {
        let to_secret = k256::SecretKey::random(&mut OsRng);
        let to_identity = Secp256k1Identity::from_private_key(to_secret.clone());
        let expiry = current_epoch() + DELEGATION_MAX_AGE;
        let expiry_ns = expiry.as_nanos() as u64;
        let delegation = Delegation {
            pubkey: to_identity.public_key().unwrap(),
            expiration: expiry_ns,
            targets: None,
        };
        let sig = from.sign_delegation(&delegation).unwrap();
        let signed_delegation = SignedDelegation {
            delegation,
            signature: sig.signature.unwrap(),
        };

        Self {
            from_key: sig.public_key.unwrap(),
            to_secret: to_secret.to_jwk(),
            delegation_chain: vec![signed_delegation],
        }
    }
}

fn set_cookies(resp: &ResponseOptions, jar: impl IntoResponse) {
    let resp_jar = jar.into_response();
    for cookie in resp_jar
        .headers()
        .get_all(header::SET_COOKIE)
        .into_iter()
        .cloned()
    {
        resp.append_header(header::SET_COOKIE, cookie);
    }
}

async fn extract_principal_from_cookie(
    jar: &SignedCookieJar,
) -> Result<Option<Principal>, ServerFnError> {
    let Some(cookie) = jar.get(REFRESH_TOKEN_COOKIE) else {
        return Ok(None);
    };
    let token: RefreshToken = serde_json::from_str(cookie.value())?;
    if current_epoch().as_millis() > token.expiry_epoch_ms {
        return Ok(None);
    }
    Ok(Some(token.principal))
}

async fn fetch_identity_from_kv(
    kv: &KVStoreImpl,
    principal: Principal,
) -> Result<Option<k256::SecretKey>, ServerFnError> {
    let Some(identity_jwk) = kv.read(principal.to_text()).await? else {
        return Ok(None);
    };

    Ok(Some(k256::SecretKey::from_jwk_str(&identity_jwk)?))
}

pub async fn try_extract_identity(
    jar: &SignedCookieJar,
    kv: &KVStoreImpl,
) -> Result<Option<k256::SecretKey>, ServerFnError> {
    let Some(principal) = extract_principal_from_cookie(jar).await? else {
        return Ok(None);
    };
    fetch_identity_from_kv(kv, principal).await
}


pub async fn webhook_kv_update_impl(uid: String) -> Result<(),ServerFnError>{
    let kv: KVStoreImpl = expect_context();

    Ok(kv.write(uid, false.to_string()).await?)
}


async fn generate_and_save_identity(kv: &KVStoreImpl) -> Result<Secp256k1Identity, ServerFnError> {
    let base_identity_key = k256::SecretKey::random(&mut OsRng);
    let base_identity = Secp256k1Identity::from_private_key(base_identity_key.clone());
    let principal = base_identity.sender().unwrap();

    let base_jwk = base_identity_key.to_jwk_string();
    kv.write(principal.to_text(), base_jwk.to_string()).await?;
    Ok(base_identity)
}

pub async fn update_user_identity(
    response_opts: &ResponseOptions,
    mut jar: SignedCookieJar,
    identity: impl Identity,
) -> Result<DelegatedIdentityWire, ServerFnError> {
    let refresh_max_age = REFRESH_MAX_AGE;
    let refresh_token = RefreshToken {
        principal: identity.sender().unwrap(),
        expiry_epoch_ms: (current_epoch() + refresh_max_age).as_millis(),
    };
    let refresh_token_enc = serde_json::to_string(&refresh_token)?;

    let refresh_cookie = Cookie::build((REFRESH_TOKEN_COOKIE, refresh_token_enc))
        .http_only(true)
        .secure(true)
        .path("/")
        .same_site(SameSite::None)
        .partitioned(true)
        .max_age(refresh_max_age.try_into().unwrap());

    jar = jar.add(refresh_cookie);
    set_cookies(response_opts, jar);

    Ok(DelegatedIdentityWire::delegate(&identity))
}

pub async fn extract_or_generate_identity_impl() -> Result<DelegatedIdentityWire, ServerFnError> {
    let key: Key = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    let kv: KVStoreImpl = expect_context();

    let base_identity = if let Some(identity) = try_extract_identity(&jar, &kv).await? {
        Secp256k1Identity::from_private_key(identity)
    } else {
        generate_and_save_identity(&kv).await?
    };

    let resp: ResponseOptions = expect_context();
    let delegated = update_user_identity(&resp, jar, base_identity).await?;

    Ok(delegated)
}

pub async fn logout_identity_impl() -> Result<DelegatedIdentityWire, ServerFnError> {
    let key: Key = expect_context();
    let kv: KVStoreImpl = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    let base_identity = generate_and_save_identity(&kv).await?;

    let resp: ResponseOptions = expect_context();
    let delegated = update_user_identity(&resp, jar, base_identity).await?;
    Ok(delegated)
}
