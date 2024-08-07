use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    misc::role_misc::default_roles,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::PrivacyType, role::Role,
        sort_direction::SortDirection,
    },
    Filter, Sorter,
};

use super::{
    api_error::ApiError,
    boosted::Boost,
    general_structs::{
        members::Members, metadata::Metadata, privacy::Privacy, references::References,
    },
    invite_type::InviteType,
    member::{Invite, Join},
    permission::Permission,
    relation_type::RelationType,
};

impl_storable_for!(GroupWithMembers);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupWithMembers {
    pub metadata: Metadata,
    pub privacy: Privacy,
    pub owner: Principal,
    pub created_by: Principal,
    pub members: Members,
    pub references: References,
    pub matrix_space_id: String,
    pub wallets: HashMap<Principal, String>,
    pub events: Vec<u64>,
    pub is_deleted: bool,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Default for GroupWithMembers {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            privacy: Default::default(),
            owner: Principal::anonymous(),
            created_by: Principal::anonymous(),
            members: Default::default(),
            references: Default::default(),
            is_deleted: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            matrix_space_id: Default::default(),
            events: Default::default(),
            wallets: Default::default(),
        }
    }
}

impl From<PostGroup> for GroupWithMembers {
    fn from(group: PostGroup) -> Self {
        Self {
            metadata: Metadata {
                name: group.name,
                description: group.description,
                website: group.website,
                location: group.location,
                image: group.image,
                banner_image: group.banner_image,
            },
            privacy: Privacy {
                privacy_type: group.privacy,
                privacy_gated_type_amount: group.privacy_gated_type_amount,
            },
            owner: caller(),
            created_by: caller(),
            members: Members::new_with_owner(caller()),
            references: References {
                notification_id: Default::default(),
                tags: group.tags,
            },
            is_deleted: false,
            updated_on: time(),
            created_on: time(),
            matrix_space_id: group.matrix_space_id,
            events: Default::default(),
            wallets: Default::default(),
        }
    }
}

impl GroupWithMembers {
    pub fn update(&mut self, group: UpdateGroup) -> Self {
        self.metadata.name = group.name;
        self.metadata.description = group.description;
        self.metadata.website = group.website;
        self.metadata.location = group.location;
        self.metadata.image = group.image;
        self.metadata.banner_image = group.banner_image;
        self.privacy.privacy_type = group.privacy;
        self.privacy.privacy_gated_type_amount = group.privacy_gated_type_amount;
        self.references.tags = group.tags;
        self.updated_on = time();
        self.clone()
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.members.set_owner(owner);
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_members(&self) -> Vec<Principal> {
        self.members.members.keys().cloned().collect()
    }

    pub fn remove_member(&mut self, member: Principal) {
        self.members.members.remove(&member);
    }

    pub fn add_member(&mut self, member: Principal) {
        self.members.members.insert(member, Join::default());
    }

    pub fn set_member_role(&mut self, member: Principal, role: String) {
        if let Some(member) = self.members.members.get_mut(&member) {
            member.set_role(role);
        }
    }

    pub fn get_invites(&self) -> Vec<Principal> {
        self.members.invites.keys().cloned().collect()
    }

    pub fn add_invite(
        &mut self,
        member: Principal,
        invite_type: InviteType,
        notification_id: Option<u64>,
    ) {
        self.members.invites.insert(
            member,
            Invite {
                notification_id,
                invite_type,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn remove_invite(&mut self, member: Principal) {
        self.members.invites.remove(&member);
    }

    pub fn convert_invite_to_member(&mut self, principal: Principal) {
        if self.members.invites.remove(&principal).is_some() {
            self.members.members.insert(principal, Join::default());
        }
    }

    pub fn get_event_ids(&self) -> Vec<u64> {
        self.events.clone()
    }

    pub fn add_event(&mut self, event_id: u64) {
        self.events.push(event_id);
    }

    pub fn remove_event(&mut self, event_id: u64) {
        self.events.retain(|&id| id != event_id);
    }

    pub fn get_roles(&self) -> Vec<Role> {
        // set the default protected roles
        let mut roles = default_roles();

        // append the custom roles stored on the group
        roles.append(&mut self.members.roles.clone());
        roles
    }

    pub fn get_role_permissions(&self, role: String) -> Vec<Permission> {
        let roles = self.get_roles();
        let role = roles.iter().find(|r| r.name == role);
        if let Some(role) = role {
            return role.permissions.clone();
        }
        vec![]
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.references.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.references.notification_id = None;
    }

    pub fn add_special_member(&mut self, member: Principal, relation: RelationType) {
        self.members
            .special_members
            .insert(member, relation.to_string());
    }

    pub fn remove_special_member(&mut self, member: Principal) {
        self.members.special_members.remove(&member);
    }

    pub fn is_banned_member(&self, member: Principal) -> bool {
        self.members
            .special_members
            .get(&member)
            .map(|relation| relation == &RelationType::Blocked.to_string())
            .unwrap_or(false)
    }
}

pub type GroupEntry = (u64, GroupWithMembers);

#[derive(Clone, CandidType, Deserialize)]
pub struct PostGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub matrix_space_id: String,
    pub location: Location,
    pub privacy: PrivacyType,
    pub privacy_gated_type_amount: Option<u64>,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct UpdateGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: PrivacyType,
    pub image: Asset,
    pub privacy_gated_type_amount: Option<u64>,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: PrivacyType,
    pub created_by: Principal,
    pub owner: Principal,
    pub matrix_space_id: String,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub roles: Vec<Role>,
    pub wallets: Vec<(Principal, String)>,
    pub is_deleted: bool,
    pub privacy_gated_type_amount: Option<u64>,
    pub updated_on: u64,
    pub created_on: u64,
    pub boosted: Option<Boost>,
    pub events_count: u64,
    pub members_count: u64,
}

impl GroupResponse {
    pub fn new(id: u64, group: GroupWithMembers, boosted: Option<Boost>) -> Self {
        let mut roles = default_roles();
        roles.append(&mut group.members.roles.clone());
        Self {
            id,
            name: group.metadata.name,
            description: group.metadata.description,
            website: group.metadata.website,
            location: group.metadata.location,
            privacy: group.privacy.privacy_type,
            created_by: group.created_by,
            owner: group.owner,
            matrix_space_id: group.matrix_space_id,
            image: group.metadata.image,
            banner_image: group.metadata.banner_image,
            tags: group.references.tags,
            roles,
            wallets: group.wallets.into_iter().collect(),
            is_deleted: group.is_deleted,
            privacy_gated_type_amount: group.privacy.privacy_gated_type_amount,
            updated_on: group.updated_on,
            created_on: group.created_on,
            boosted,
            events_count: group.events.len() as u64,
            members_count: group.members.members.len() as u64,
        }
    }

    pub fn from_result(
        group_result: Result<(u64, GroupWithMembers), ApiError>,
        boosted: Option<Boost>,
    ) -> Result<Self, ApiError> {
        match group_result {
            Err(err) => Err(err),
            Ok((id, group)) => Ok(Self::new(id, group, boosted)),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum GroupSort {
    Name(SortDirection),
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
    MemberCount(SortDirection),
}

impl Default for GroupSort {
    fn default() -> Self {
        GroupSort::CreatedOn(SortDirection::Asc)
    }
}

impl Sorter<u64, GroupWithMembers> for GroupSort {
    fn sort(&self, groups: Vec<(u64, GroupWithMembers)>) -> Vec<(u64, GroupWithMembers)> {
        let mut groups: Vec<(u64, GroupWithMembers)> = groups.into_iter().collect();
        use GroupSort::*;
        use SortDirection::*;
        match self {
            Name(Asc) => groups.sort_by(|(_, a), (_, b)| {
                a.metadata
                    .name
                    .to_lowercase()
                    .cmp(&b.metadata.name.to_lowercase())
            }),
            Name(Desc) => groups.sort_by(|(_, a), (_, b)| {
                b.metadata
                    .name
                    .to_lowercase()
                    .cmp(&a.metadata.name.to_lowercase())
            }),
            CreatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.created_on.cmp(&b.created_on)),
            CreatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.created_on.cmp(&a.created_on)),
            UpdatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.updated_on.cmp(&b.updated_on)),
            UpdatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.updated_on.cmp(&a.updated_on)),
            MemberCount(Asc) => groups
                .sort_by(|(_, a), (_, b)| a.members.members.len().cmp(&b.members.members.len())),
            MemberCount(Desc) => groups
                .sort_by(|(_, a), (_, b)| b.members.members.len().cmp(&a.members.members.len())),
        }
        groups
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct GroupsCount {
    pub total: u64,
    pub joined: u64,
    pub invited: u64,
    pub starred: u64,
    pub new: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub enum GroupFilter {
    #[default]
    None,
    Name(String),
    Owner(Principal),
    Ids(Vec<u64>),
    Tag(u32),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}

impl Filter<u64, GroupWithMembers> for GroupFilter {
    fn matches(&self, id: &u64, group: &GroupWithMembers) -> bool {
        use GroupFilter::*;
        match self {
            None => true,
            Name(name) => group
                .metadata
                .name
                .to_lowercase()
                .contains(&name.to_lowercase()),
            Owner(owner) => group.owner == *owner,
            Ids(ids) => ids.contains(id),
            Tag(tag) => group.references.tags.contains(tag),
            UpdatedOn(range) => {
                if range.end_date() > 0 {
                    range.is_within(group.updated_on)
                } else {
                    range.is_after_start_date(group.updated_on)
                }
            }
            CreatedOn(range) => {
                if range.end_date() > 0 {
                    range.is_within(group.updated_on)
                } else {
                    range.is_after_start_date(group.updated_on)
                }
            }
        }
    }
}

impl From<GroupFilter> for Vec<GroupFilter> {
    fn from(val: GroupFilter) -> Self {
        vec![val]
    }
}
