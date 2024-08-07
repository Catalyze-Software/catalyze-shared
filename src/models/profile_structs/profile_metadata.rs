use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{asset::Asset, profile_with_refs::PostProfile};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct ProfileMetadata {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub website: String,
}

impl From<PostProfile> for ProfileMetadata {
    fn from(profile: PostProfile) -> Self {
        Self {
            username: profile.username,
            display_name: profile.display_name,
            first_name: profile.first_name,
            last_name: profile.last_name,
            about: "".to_string(),
            email: "".to_string(),
            date_of_birth: 0,
            city: "".to_string(),
            state_or_province: "".to_string(),
            country: "".to_string(),
            profile_image: Default::default(),
            banner_image: Default::default(),
            website: "".to_string(),
        }
    }
}
