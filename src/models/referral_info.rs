use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

impl_storable_for!(ReferralInfo);

#[derive(Deserialize, CandidType, Clone)]
pub struct ReferralInfo {
    pub referrer: Principal,
    pub referral: Principal,
    pub created_at: u64,
}
