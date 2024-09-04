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
    general_structs::privacy::Privacy,
    profile_structs::{
        profile_documents::ProfileDocuments, profile_metadata::ProfileMetadata,
        profile_references::ProfileReferences,
    },
    referral::Referral,
    subject::Subject,
    wallet::WalletResponse,
};

impl_storable_for!(ProfileWithRefs);

#[derive(Clone, Default, Debug, CandidType, Deserialize, Serialize)]
pub struct ProfileWithRefs {
    pub metadata: ProfileMetadata,
    pub documents: ProfileDocuments,
    pub application_role: ApplicationRole,
    pub privacy: Privacy,
    pub references: ProfileReferences,
    pub extra: Option<String>,
    pub notification_id: Option<u64>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl From<PostProfile> for ProfileWithRefs {
    fn from(profile: PostProfile) -> Self {
        Self {
            metadata: ProfileMetadata::from(profile),
            documents: ProfileDocuments::default(),
            application_role: ApplicationRole::default(),
            privacy: Privacy::default(),
            references: ProfileReferences::default(),
            extra: None,
            notification_id: None,
            updated_on: time(),
            created_on: time(),
        }
    }
}

impl ProfileWithRefs {
    pub fn update(self, profile: UpdateProfile) -> Self {
        Self {
            metadata: ProfileMetadata {
                display_name: profile.display_name,
                first_name: profile.first_name,
                last_name: profile.last_name,
                about: profile.about,
                email: profile.email.unwrap_or_default(),
                date_of_birth: profile.date_of_birth,
                city: profile.city,
                state_or_province: profile.state_or_province,
                country: profile.country,
                profile_image: profile.profile_image,
                banner_image: profile.banner_image,
                website: profile.website,
                username: self.metadata.username,
            },
            documents: self.documents,
            privacy: profile.privacy,
            extra: profile.extra,
            updated_on: time(),
            created_on: self.created_on,
            application_role: self.application_role,
            references: self.references,
            notification_id: self.notification_id,
        }
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }

    pub fn is_starred(&self, subject: &Subject) -> bool {
        self.references.starred.contains(subject)
    }

    pub fn is_pinned(&self, subject: &Subject) -> bool {
        self.references.pinned.contains(subject)
    }

    pub fn remove_pinned(&mut self, subject: &Subject) {
        self.references.pinned.retain(|s| s != subject);
    }

    pub fn remove_starred(&mut self, subject: &Subject) {
        self.references.starred.retain(|s| s != subject);
    }

    pub fn add_group(&mut self, group_id: u64) {
        self.references.groups.push(group_id);
    }

    pub fn is_group_member(&self, group_id: u64) -> bool {
        self.references.groups.contains(&group_id)
    }

    pub fn get_group_ids(&self) -> Vec<u64> {
        self.references.groups.clone()
    }

    pub fn remove_group(&mut self, group_id: u64) {
        self.references.groups.retain(|id| id != &group_id);
    }

    pub fn add_event(&mut self, event_id: u64) {
        self.references.events.push(event_id);
    }

    pub fn is_event_attendee(&self, event_id: u64) -> bool {
        self.references.events.contains(&event_id)
    }

    pub fn get_event_ids(&self) -> Vec<u64> {
        self.references.events.clone()
    }

    pub fn remove_event(&mut self, event_id: u64) {
        self.references.events.retain(|id| id != &event_id);
    }

    pub fn add_referral(&mut self, principal: Principal) {
        self.references
            .referrals
            .insert(principal, Referral::default());
    }

    pub fn remove_referral(&mut self, principal: Principal) {
        self.references.referrals.remove(&principal);
    }

    pub fn is_referral_exists(&self, principal: Principal) -> bool {
        self.references.referrals.contains_key(&principal)
    }

    pub fn is_referral_expired(&self, principal: Principal) -> bool {
        self.references
            .referrals
            .get(&principal)
            .map_or(false, |referral| referral.is_expired())
    }
}

pub type ProfileEntry = (Principal, ProfileWithRefs);

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct PostProfile {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: Privacy,
    pub extra: String,
    pub referrer: Option<Principal>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateProfile {
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: Privacy,
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
    pub extra: Option<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub principal: Principal,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: Privacy,
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
    pub extra: Option<String>,
    pub updated_on: u64,
    pub created_on: u64,
    pub referrer: Option<Principal>,
}

impl ProfileResponse {
    pub fn new(principal: Principal, profile: ProfileWithRefs) -> Self {
        let wallets = profile
            .references
            .wallets
            .into_iter()
            .map(|(address, wallet)| WalletResponse {
                provider: wallet.provider,
                address,
                is_primary: wallet.is_primary,
            })
            .collect();

        Self {
            principal,
            username: profile.metadata.username,
            display_name: profile.metadata.display_name,
            about: profile.metadata.about,
            city: profile.metadata.city,
            country: profile.metadata.country,
            website: profile.metadata.website,
            skills: profile.references.skills,
            interests: profile.references.interests,
            causes: profile.references.causes,
            email: profile.metadata.email,
            application_role: profile.application_role,
            first_name: profile.metadata.first_name,
            last_name: profile.metadata.last_name,
            privacy: profile.privacy,
            date_of_birth: profile.metadata.date_of_birth,
            state_or_province: profile.metadata.state_or_province,
            profile_image: profile.metadata.profile_image,
            banner_image: profile.metadata.banner_image,
            code_of_conduct: profile.documents.code_of_conduct,
            privacy_policy: profile.documents.privacy_policy,
            terms_of_service: profile.documents.terms_of_service,
            wallets,
            pinned: profile.references.pinned,
            starred: profile.references.starred,
            extra: profile.extra,
            updated_on: profile.updated_on,
            created_on: profile.created_on,
            referrer: profile.references.referrer,
        }
    }

    pub fn to_result(self) -> CanisterResult<Self> {
        Ok(self)
    }
}

impl From<ProfileEntry> for ProfileResponse {
    fn from((principal, profile): ProfileEntry) -> Self {
        Self::new(principal, profile)
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

impl Sorter<Principal, ProfileWithRefs> for ProfileSort {
    fn sort(
        &self,
        profiles: Vec<(Principal, ProfileWithRefs)>,
    ) -> Vec<(Principal, ProfileWithRefs)> {
        let mut profiles = profiles;

        use ProfileSort::*;
        use SortDirection::*;
        match self {
            CreatedOn(Asc) => profiles.sort_by(|a, b| a.1.created_on.cmp(&b.1.created_on)),
            CreatedOn(Desc) => profiles.sort_by(|a, b| b.1.created_on.cmp(&a.1.created_on)),
            UpdatedOn(Asc) => profiles.sort_by(|a, b| a.1.updated_on.cmp(&b.1.updated_on)),
            UpdatedOn(Desc) => profiles.sort_by(|a, b| b.1.updated_on.cmp(&a.1.updated_on)),
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

impl Filter<Principal, ProfileWithRefs> for ProfileFilter {
    fn matches(&self, _key: &Principal, value: &ProfileWithRefs) -> bool {
        use ProfileFilter::*;
        match self {
            Username(username) => eq_str(&value.metadata.username, username),
            DisplayName(display_name) => eq_str(&value.metadata.display_name, display_name),
            FirstName(first_name) => eq_str(&value.metadata.first_name, first_name),
            LastName(last_name) => eq_str(&value.metadata.last_name, last_name),
            Email(email) => eq_str(&value.metadata.email, email),
            City(city) => eq_str(&value.metadata.city, city),
            StateOrProvince(state_or_province) => {
                eq_str(&value.metadata.state_or_province, state_or_province)
            }
            Country(country) => eq_str(&value.metadata.country, country),
            UpdatedOn(date_range) => date_range.is_within(value.updated_on),
            Skill(skill) => value.references.skills.contains(skill),
            Interest(interest) => value.references.interests.contains(interest),
            Cause(cause) => value.references.causes.contains(cause),
            CreatedOn(date_range) => date_range.is_within(value.created_on),
        }
    }
}

pub type ProfileWithRefsEntry = (Principal, ProfileWithRefs);
