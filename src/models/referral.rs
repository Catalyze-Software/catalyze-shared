use candid::CandidType;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(Referral);

pub const REFERRAL_EXPIRATION: u64 = 604800000000000; // 1 week in nanoseconds

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Referral {
    pub created_at: u64,
}

impl Default for Referral {
    fn default() -> Self {
        Self { created_at: time() }
    }
}

impl Referral {
    pub fn is_expired(&self) -> bool {
        time() - self.created_at > REFERRAL_EXPIRATION
    }
}

#[derive(Default, Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ReferralStatus {
    #[default]
    Created,
    Accepted,
}
