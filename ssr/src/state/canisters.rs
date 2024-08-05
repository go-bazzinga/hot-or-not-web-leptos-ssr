use std::sync::Arc;

use candid::Principal;
use ic_agent::{identity::DelegatedIdentity, AgentError, Identity};
use leptos::*;
use yral_metadata_client::MetadataClient;
use yral_metadata_types::UserMetadata;

use crate::{
    auth::DelegatedIdentityWire,
    canister::{
        individual_user_template::{IndividualUserTemplate, Result9, UserCanisterDetails},
        platform_orchestrator::PlatformOrchestrator,
        post_cache::PostCache,
        user_index::UserIndex,
        PLATFORM_ORCHESTRATOR_ID, POST_CACHE_ID,
    },
    consts::METADATA_API_BASE,
    utils::{ic::AgentWrapper, profile::ProfileDetails, MockPartialEq},
};

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: AgentWrapper,
    id: Option<Arc<DelegatedIdentity>>,
    metadata_client: MetadataClient<false>,
    user_canister: Principal,
    expiry: u64,
    profile_details: Option<ProfileDetails>,
}

impl Default for Canisters<false> {
    fn default() -> Self {
        Self {
            agent: AgentWrapper::build(|b| b),
            id: None,
            metadata_client: MetadataClient::with_base_url(METADATA_API_BASE.clone()),
            user_canister: Principal::anonymous(),
            expiry: 0,
            profile_details: None,
        }
    }
}

impl Canisters<true> {
    pub fn authenticated(id: DelegatedIdentity) -> Canisters<true> {
        let expiry = id
            .delegation_chain()
            .iter()
            .fold(u64::MAX, |prev_expiry, del| {
                del.delegation.expiration.min(prev_expiry)
            });
        let id = Arc::new(id);

        Canisters {
            agent: AgentWrapper::build(|b| b.with_arc_identity(id.clone())),
            metadata_client: MetadataClient::with_base_url(METADATA_API_BASE.clone()),
            id: Some(id),
            user_canister: Principal::anonymous(),
            expiry,
            profile_details: None,
        }
    }

    pub fn expiry_ns(&self) -> u64 {
        self.expiry
    }

    pub fn identity(&self) -> &DelegatedIdentity {
        self.id
            .as_ref()
            .expect("Authenticated canisters must have an identity")
    }

    // pub fn user_canister(&self) -> Principal {
    //     self.user_canister
    // }

    pub async fn authenticated_user(&self) -> Result<IndividualUserTemplate<'_>, AgentError> {
        self.individual_user(self.user_canister).await
    }

    pub fn profile_details(&self) -> ProfileDetails {
        self.profile_details
            .clone()
            .expect("Authenticated canisters must have profile details")
    }

    pub fn user_principal(&self) -> Principal {
        self.identity()
            .sender()
            .expect("expect principal to be present")
    }
}

impl<const A: bool> Canisters<A> {
    pub fn user_canister(&self) -> Principal {
        self.user_canister
    }

    pub async fn post_cache(&self) -> Result<PostCache<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(PostCache(POST_CACHE_ID, agent))
    }

    pub async fn individual_user(
        &self,
        user_canister: Principal,
    ) -> Result<IndividualUserTemplate<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(IndividualUserTemplate(user_canister, agent))
    }

    pub async fn user_index_with(
        &self,
        subnet_principal: Principal,
    ) -> Result<UserIndex<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(UserIndex(subnet_principal, agent))
    }

    pub async fn orchestrator(&self) -> Result<PlatformOrchestrator<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(PlatformOrchestrator(PLATFORM_ORCHESTRATOR_ID, agent))
    }

    pub async fn get_individual_canister_by_user_principal(
        &self,
        user_principal: Principal,
    ) -> Result<Option<Principal>, ServerFnError> {
        let meta = self
            .metadata_client
            .get_user_metadata(user_principal)
            .await?;
        Ok(meta.map(|m| m.user_canister_id))
    }

    async fn subnet_indexes(&self) -> Result<Vec<Principal>, AgentError> {
        #[cfg(any(feature = "local-bin", feature = "local-lib"))]
        {
            use crate::canister::USER_INDEX_ID;
            Ok(vec![USER_INDEX_ID])
        }
        #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
        {
            use std::collections::HashSet;
            // TODO: this is temporary
            let blacklisted =
                HashSet::from([Principal::from_text("rimrc-piaaa-aaaao-aaljq-cai").unwrap()]);
            let orchestrator = self.orchestrator().await?;
            Ok(orchestrator
                .get_all_available_subnet_orchestrators()
                .await?
                .into_iter()
                .filter(|subnet| !blacklisted.contains(subnet))
                .collect())
        }
    }
}

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

async fn create_individual_canister(
    canisters: &Canisters<true>,
) -> Result<Principal, ServerFnError> {
    let subnet_idxs = canisters.subnet_indexes().await?;

    let mut by = [0u8; 16];
    let principal = canisters.identity().sender().unwrap();
    let principal_by = principal.as_slice();
    let cnt = by.len().min(principal_by.len());
    by[..cnt].copy_from_slice(&principal_by[..cnt]);

    let discrim = u128::from_be_bytes(by);
    let subnet_idx = subnet_idxs[(discrim % subnet_idxs.len() as u128) as usize];
    let idx = canisters.user_index_with(subnet_idx).await?;
    let user_canister = idx
        .get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer()
        .await?;

    canisters
        .metadata_client
        .set_user_metadata(
            canisters.identity(),
            UserMetadata {
                user_canister_id: user_canister,
                user_name: "".into(),
            },
        )
        .await?;

    Ok(user_canister)
}

pub async fn do_canister_auth(
    auth: DelegatedIdentityWire,
    referrer: Option<Principal>,
) -> Result<Canisters<true>, ServerFnError> {
    let id: DelegatedIdentity = auth.clone().try_into()?;
    let mut canisters = Canisters::<true>::authenticated(id);

    canisters.user_canister = if let Some(user_canister) = canisters
        .get_individual_canister_by_user_principal(canisters.identity().sender().unwrap())
        .await?
    {
        user_canister
    } else {
        create_individual_canister(&canisters).await?
    };

    let user = canisters.authenticated_user().await?;

    if let Some(referrer_principal_id) = referrer {
        let referrer_canister = canisters
            .get_individual_canister_by_user_principal(referrer_principal_id)
            .await?;
        if let Some(referrer_canister_id) = referrer_canister {
            user.update_referrer_details(UserCanisterDetails {
                user_canister_id: referrer_canister_id,
                profile_owner: referrer_principal_id,
            })
            .await?;
        }
    }

    match user
        .update_last_access_time()
        .await
        .map_err(|e| e.to_string())
    {
        Ok(Result9::Ok(_)) => (),
        Err(e) | Ok(Result9::Err(e)) => log::warn!("Failed to update last access time: {}", e),
    }
    canisters.profile_details = Some(user.get_profile_details().await?.into());

    Ok(canisters)
}

pub type AuthCansResource =
    Resource<MockPartialEq<Option<DelegatedIdentityWire>>, Result<Canisters<true>, ServerFnError>>;

pub fn authenticated_canisters() -> AuthCansResource {
    expect_context()
}

pub fn auth_canisters_store() -> RwSignal<Option<Canisters<true>>> {
    expect_context()
}
