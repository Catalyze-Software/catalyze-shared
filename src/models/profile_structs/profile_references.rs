use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::{subject::Subject, wallet::Wallet};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct ProfileReferences {
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub starred: Vec<Subject>,
    pub pinned: Vec<Subject>,
    pub groups: Vec<u64>,
    pub events: Vec<u64>,
    pub wallets: HashMap<String, Wallet>,
    pub relations: HashMap<Principal, String>,
}
