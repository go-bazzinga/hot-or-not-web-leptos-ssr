use candid::Principal;

use crate::canister::{AGENT_URL, post_cache::{PostCache, self}, individual_user_template::{IndividualUserTemplate, self}};


#[derive(Debug, Clone)]
pub struct Canisters {
    agent: ic_agent::Agent,
}

impl Default for Canisters {
    fn default() -> Self {
        Self {
            agent: ic_agent::Agent::builder()
                .with_url(AGENT_URL)
                .build()
                .unwrap(),
        }
    }
}

impl Canisters {
    pub fn post_cache(&self) -> PostCache<'_> {
        PostCache(post_cache::CANISTER_ID, &self.agent)
    }

    pub fn individual_user(&self, user_canister: Principal) -> IndividualUserTemplate<'_> {
        IndividualUserTemplate(user_canister, &self.agent)
    }
}