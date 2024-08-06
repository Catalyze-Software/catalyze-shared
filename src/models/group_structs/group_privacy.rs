use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::privacy::Privacy;

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct GroupPrivacy {
    pub privacy: Privacy,
    pub privacy_gated_type_amount: Option<u64>,
}
