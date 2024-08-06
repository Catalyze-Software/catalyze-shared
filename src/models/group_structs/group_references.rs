use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct GroupReferences {
    pub matrix_space_id: String,
    pub wallets: HashMap<Principal, String>,
    pub notification_id: Option<u64>,
    pub events: Vec<u64>,
    pub tags: Vec<u32>,
}
