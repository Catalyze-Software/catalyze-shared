use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{
        application_role::ApplicationRole, asset::Asset, date_range::DateRange,
        sort_direction::SortDirection,
    },
    str::eq_str,
    CanisterResult, Filter, Sorter,
};

use super::{
    document_details::DocumentDetails,
    profile_privacy::ProfilePrivacy,
    subject::Subject,
    wallet::{Wallet, WalletResponse},
};

impl_storable_for!(Profile);

#[derive(Clone, Default, Debug, CandidType, Deserialize, Serialize)]
pub struct Profile {
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub code_of_conduct: Option<DocumentDetails>,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
    pub wallets: HashMap<Principal, Wallet>,
    pub starred: Vec<Subject>,
    pub pinned: Vec<Subject>,
    pub relations: HashMap<Principal, String>,
    pub extra: String,
    pub notification_id: Option<u64>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Profile {
    pub fn remove_pinned(&mut self, subject: &Subject) {
        self.pinned.retain(|s| s != subject);
    }

    pub fn remove_starred(&mut self, subject: &Subject) {
        self.starred.retain(|s| s != subject);
    }

    pub fn update(self, profile: UpdateProfile) -> Self {
        Self {
            username: self.username,
            display_name: profile.display_name,
            application_role: self.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            about: profile.about,
            email: profile.email.unwrap_or("".to_string()),
            date_of_birth: profile.date_of_birth,
            city: profile.city,
            state_or_province: profile.state_or_province,
            country: profile.country,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            website: profile.website,
            wallets: self.wallets,
            starred: self.starred,
            pinned: self.pinned,
            relations: self.relations,
            code_of_conduct: self.code_of_conduct,
            extra: profile.extra,
            updated_on: time(),
            notification_id: self.notification_id,
            created_on: self.created_on,
            privacy_policy: self.privacy_policy,
            terms_of_service: self.terms_of_service,
        }
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }

    pub fn is_starred(&self, subject: &Subject) -> bool {
        self.starred.contains(subject)
    }

    pub fn is_pinned(&self, subject: &Subject) -> bool {
        self.pinned.contains(subject)
    }
}

impl From<PostProfile> for Profile {
    fn from(profile: PostProfile) -> Self {
        Self {
            username: profile.username,
            display_name: profile.display_name,
            application_role: ApplicationRole::default(),
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            about: "".to_string(),
            email: "".to_string(),
            date_of_birth: 0,
            city: "".to_string(),
            state_or_province: "".to_string(),
            country: "".to_string(),
            profile_image: Asset::None,
            banner_image: Asset::None,
            skills: vec![],
            interests: vec![],
            causes: vec![],
            website: "".to_string(),
            wallets: HashMap::new(),
            starred: Vec::new(),
            pinned: Vec::new(),
            relations: HashMap::new(),
            code_of_conduct: None,
            extra: profile.extra,
            updated_on: time(),
            created_on: time(),
            notification_id: None,
            privacy_policy: None,
            terms_of_service: None,
        }
    }
}

pub type ProfileEntry = (Principal, Profile);

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct PostProfile {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub extra: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateProfile {
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: Option<String>,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub extra: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub principal: Principal,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub code_of_conduct: Option<DocumentDetails>,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
    pub pinned: Vec<Subject>,
    pub starred: Vec<Subject>,
    pub wallets: Vec<WalletResponse>,
    pub extra: String,
    pub updated_on: u64,
    pub created_on: u64,
}

impl ProfileResponse {
    pub fn new(principal: Principal, profile: Profile) -> Self {
        let wallets = profile
            .wallets
            .into_iter()
            .map(|(principal, wallet)| WalletResponse {
                provider: wallet.provider,
                principal,
                is_primary: wallet.is_primary,
            })
            .collect();

        Self {
            principal,
            username: profile.username,
            display_name: profile.display_name,
            about: profile.about,
            city: profile.city,
            country: profile.country,
            website: profile.website,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            email: profile.email,
            application_role: profile.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            date_of_birth: profile.date_of_birth,
            state_or_province: profile.state_or_province,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            code_of_conduct: profile.code_of_conduct,
            privacy_policy: profile.privacy_policy,
            terms_of_service: profile.terms_of_service,
            wallets,
            pinned: profile.pinned,
            starred: profile.starred,
            extra: profile.extra,
            updated_on: profile.updated_on,
            created_on: profile.created_on,
        }
    }

    pub fn to_result(self) -> CanisterResult<Self> {
        Ok(self)
    }
}

impl From<ProfileEntry> for ProfileResponse {
    fn from((principal, profile): ProfileEntry) -> Self {
        let wallets = profile
            .wallets
            .into_iter()
            .map(|(principal, wallet)| WalletResponse {
                provider: wallet.provider,
                principal,
                is_primary: wallet.is_primary,
            })
            .collect();

        Self {
            principal,
            username: profile.username,
            display_name: profile.display_name,
            about: profile.about,
            city: profile.city,
            country: profile.country,
            website: profile.website,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            email: profile.email,
            application_role: profile.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            date_of_birth: profile.date_of_birth,
            state_or_province: profile.state_or_province,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            code_of_conduct: profile.code_of_conduct,
            privacy_policy: profile.privacy_policy,
            terms_of_service: profile.terms_of_service,
            wallets,
            pinned: profile.pinned,
            starred: profile.starred,
            extra: profile.extra,
            updated_on: profile.updated_on,
            created_on: profile.created_on,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

impl Default for ProfileSort {
    fn default() -> Self {
        ProfileSort::CreatedOn(SortDirection::default())
    }
}

impl Sorter<Principal, Profile> for ProfileSort {
    fn sort(&self, profiles: Vec<(Principal, Profile)>) -> Vec<(Principal, Profile)> {
        let mut profiles = profiles;

        match self {
            ProfileSort::CreatedOn(SortDirection::Asc) => {
                profiles.sort_by(|a, b| a.1.created_on.cmp(&b.1.created_on))
            }
            ProfileSort::CreatedOn(SortDirection::Desc) => {
                profiles.sort_by(|a, b| b.1.created_on.cmp(&a.1.created_on))
            }
            ProfileSort::UpdatedOn(SortDirection::Asc) => {
                profiles.sort_by(|a, b| a.1.updated_on.cmp(&b.1.updated_on))
            }
            ProfileSort::UpdatedOn(SortDirection::Desc) => {
                profiles.sort_by(|a, b| b.1.updated_on.cmp(&a.1.updated_on))
            }
        }
        profiles
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileFilter {
    Username(String),
    DisplayName(String),
    FirstName(String),
    LastName(String),
    Email(String),
    City(String),
    StateOrProvince(String),
    Country(String),
    UpdatedOn(DateRange),
    Skill(u32),
    Interest(u32),
    Cause(u32),
    CreatedOn(DateRange),
}

impl Filter<Principal, Profile> for ProfileFilter {
    fn matches(&self, _key: &Principal, value: &Profile) -> bool {
        match self {
            ProfileFilter::Username(username) => eq_str(&value.username, username),
            ProfileFilter::DisplayName(display_name) => eq_str(&value.display_name, display_name),
            ProfileFilter::FirstName(first_name) => eq_str(&value.first_name, first_name),
            ProfileFilter::LastName(last_name) => eq_str(&value.last_name, last_name),
            ProfileFilter::Email(email) => eq_str(&value.email, email),
            ProfileFilter::City(city) => eq_str(&value.city, city),
            ProfileFilter::StateOrProvince(state_or_province) => {
                eq_str(&value.state_or_province, state_or_province)
            }
            ProfileFilter::Country(country) => eq_str(&value.country, country),
            ProfileFilter::UpdatedOn(date_range) => date_range.is_within(value.updated_on),
            ProfileFilter::Skill(skill) => value.skills.contains(skill),
            ProfileFilter::Interest(interest) => value.interests.contains(interest),
            ProfileFilter::Cause(cause) => value.causes.contains(cause),
            ProfileFilter::CreatedOn(date_range) => date_range.is_within(value.created_on),
        }
    }
}

impl From<ProfileFilter> for Vec<ProfileFilter> {
    fn from(val: ProfileFilter) -> Self {
        vec![val]
    }
}
