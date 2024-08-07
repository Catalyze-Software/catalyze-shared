use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::document_details::DocumentDetails;

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct ProfileDocuments {
    pub code_of_conduct: Option<DocumentDetails>,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
}
