use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{asset::Asset, location::Location};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct GroupMetadata {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub banner_image: Asset,
}
