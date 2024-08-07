use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::privacy::PrivacyType;

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct Privacy {
    pub privacy_type: PrivacyType,
    pub privacy_gated_type_amount: Option<u64>,
}
