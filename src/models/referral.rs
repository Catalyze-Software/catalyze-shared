use candid::CandidType;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(Referral);

pub const REFERRAL_EXPIRATION: u64 = 604800000000000; // 1 week in nanoseconds

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Referral {
    pub is_accepted: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Default for Referral {
    fn default() -> Self {
        Self {
            is_accepted: false,
            created_at: time(),
            updated_at: time(),
        }
    }
}

impl Referral {
    pub fn is_accepted(&self) -> bool {
        self.is_accepted
    }

    pub fn is_expired(&self) -> bool {
        time() - self.created_at > REFERRAL_EXPIRATION
    }

    pub fn accept(&mut self) -> Self {
        self.is_accepted = true;
        self.updated_at = time();
        self.clone()
    }

    pub fn renew(&mut self) -> Self {
        self.is_accepted = false;
        self.created_at = time();
        self.updated_at = time();
        self.clone()
    }
}

#[derive(Default, Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ReferralStatus {
    #[default]
    Created,
    Accepted,
}
