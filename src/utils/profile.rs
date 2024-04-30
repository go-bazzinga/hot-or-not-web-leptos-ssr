use candid::Principal;
use ic_agent::AgentError;
use serde::{Deserialize, Serialize};

use crate::{
    canister::individual_user_template::{
        BetDirection, BetOutcomeForBetMaker, PlacedBetDetail, PostDetailsForFrontend, Result5,
        UserProfileDetailsForFrontend,
    },
    component::infinite_scroller::{CursoredDataProvider, KeyedData, PageEntry},
    consts::FALLBACK_PROPIC_BASE,
    state::canisters::Canisters,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileDetails {
    pub username: Option<String>,
    pub lifetime_earnings: u64,
    pub followers_cnt: u64,
    pub following_cnt: u64,
    pub profile_pic: Option<String>,
    pub display_name: Option<String>,
    pub principal: Principal,
    pub hots: u64,
    pub nots: u64,
}

impl From<UserProfileDetailsForFrontend> for ProfileDetails {
    fn from(user: UserProfileDetailsForFrontend) -> Self {
        Self {
            username: user.unique_user_name,
            lifetime_earnings: user.lifetime_earnings,
            followers_cnt: user.followers_count,
            following_cnt: user.following_count,
            profile_pic: user.profile_picture_url,
            display_name: user.display_name,
            principal: user.principal_id,
            hots: user.profile_stats.hot_bets_received,
            nots: user.profile_stats.not_bets_received,
        }
    }
}

fn color_from_principal(principal: Principal) -> String {
    let col_int = crc32fast::hash(principal.as_slice()) & 0xFFFFFF;
    format!("{col_int:06x}")
}

impl ProfileDetails {
    pub fn username_or_principal(&self) -> String {
        self.username
            .clone()
            .unwrap_or_else(|| self.principal.to_text())
    }

    pub fn display_name_or_fallback(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| self.username_or_principal())
    }

    pub fn profile_pic_or_random(&self) -> String {
        let propic = self.profile_pic.clone().unwrap_or_default();
        if !propic.is_empty() {
            return propic;
        }

        propic_from_principal(self.principal)
    }
}

pub fn propic_from_principal(principal: Principal) -> String {
    let background_color = color_from_principal(principal);
    format!(
        "{FALLBACK_PROPIC_BASE}?seed={}&backgroundColor={}&backgroundType=solid",
        principal.to_text(),
        background_color
    )
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PostDetails {
    pub creator: Principal,
    pub id: u64,
    pub uid: String,
    pub likes: u64,
    pub views: u64,
}

impl From<PostDetailsForFrontend> for PostDetails {
    fn from(post: PostDetailsForFrontend) -> Self {
        Self {
            creator: post.created_by_user_principal_id,
            id: post.id,
            uid: post.video_uid.clone(),
            likes: post.like_count,
            views: post.total_view_count,
        }
    }
}

impl KeyedData for PostDetails {
    type Key = (Principal, u64);

    fn key(&self) -> Self::Key {
        (self.creator, self.id)
    }
}

#[derive(Clone, Copy)]
pub enum BetOutcome {
    Won(u64),
    Draw(u64),
    Lost,
    AwaitingResult,
}

#[derive(Clone, Copy)]
pub enum BetKind {
    Hot,
    Not,
}

#[derive(Clone)]
pub struct BetDetails {
    pub outcome: BetOutcome,
    pub post_id: u64,
    pub canister_id: Principal,
    pub bet_kind: BetKind,
    pub bet_amount: u64,
}

impl From<PlacedBetDetail> for BetDetails {
    fn from(bet: PlacedBetDetail) -> Self {
        let outcome = match bet.outcome_received {
            BetOutcomeForBetMaker::Lost => BetOutcome::Lost,
            BetOutcomeForBetMaker::Draw(w) => BetOutcome::Draw(w),
            BetOutcomeForBetMaker::Won(w) => BetOutcome::Won(w),
            BetOutcomeForBetMaker::AwaitingResult => BetOutcome::AwaitingResult,
        };
        let bet_kind = match bet.bet_direction {
            BetDirection::Hot => BetKind::Hot,
            BetDirection::Not => BetKind::Not,
        };
        Self {
            outcome,
            post_id: bet.post_id,
            canister_id: bet.canister_id,
            bet_kind,
            bet_amount: bet.amount_bet,
        }
    }
}

impl KeyedData for BetDetails {
    type Key = (Principal, u64);

    fn key(&self) -> Self::Key {
        (self.canister_id, self.post_id)
    }
}

pub const PROFILE_CHUNK_SZ: usize = 10;

#[derive(Clone)]
pub struct PostsProvider {
    canisters: Canisters<false>,
    user: Principal,
}

impl PostsProvider {
    pub fn new(canisters: Canisters<false>, user: Principal) -> Self {
        Self { canisters, user }
    }
}

impl CursoredDataProvider for PostsProvider {
    type Data = PostDetails;
    type Error = AgentError;

    async fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<PostDetails>, AgentError> {
        let user = self.canisters.individual_user(self.user);
        let posts = user
            .get_posts_of_this_user_profile_with_pagination_cursor(start as u64, end as u64)
            .await?;
        let posts = match posts {
            Result5::Ok(v) => v,
            Result5::Err(_) => {
                log::warn!("failed to get posts");
                return Ok(PageEntry {
                    data: vec![],
                    end: true,
                });
            }
        };
        let list_end = posts.len() < (end - start);
        Ok(PageEntry {
            data: posts.into_iter().map(PostDetails::from).collect(),
            end: list_end,
        })
    }
}

#[derive(Clone)]
pub struct BetsProvider {
    canisters: Canisters<false>,
    user: Principal,
}

impl BetsProvider {
    pub fn new(canisters: Canisters<false>, user: Principal) -> Self {
        Self { canisters, user }
    }
}

impl CursoredDataProvider for BetsProvider {
    type Data = BetDetails;
    type Error = AgentError;

    async fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<BetDetails>, AgentError> {
        let user = self.canisters.individual_user(self.user);
        assert_eq!(end - start, 10);
        let bets = user
            .get_hot_or_not_bets_placed_by_this_profile_with_pagination(start as u64)
            .await?;
        let list_end = bets.len() < (end - start);
        Ok(PageEntry {
            data: bets.into_iter().map(PlacedBetDetail::into).collect(),
            end: list_end,
        })
    }
}
