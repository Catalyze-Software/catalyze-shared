use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct References {
    pub notification_id: Option<u64>,
    pub tags: Vec<u32>,
}
