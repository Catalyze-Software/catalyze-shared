use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    misc::role_misc::{default_roles, ADMIN_ROLE, MEMBER_ROLE, MODERATOR_ROLE, OWNER_ROLE},
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::Privacy, role::Role,
        sort_direction::SortDirection,
    },
    Filter, Sorter,
};

use super::{
    api_error::ApiError, boosted::Boost, invite_type::InviteType, permission::Permission,
    relation_type::RelationType,
};

impl_storable_for!(GroupWithMembers);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupWithMembers {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub owner: Principal,
    pub created_by: Principal,
    pub matrix_space_id: String,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub privacy_gated_type_amount: Option<u64>,
    pub roles: Vec<Role>,
    pub is_deleted: bool,
    pub notification_id: Option<u64>,
    pub special_members: HashMap<Principal, String>,
    pub wallets: HashMap<Principal, String>,
    pub members: HashMap<Principal, GroupMember>,
    pub invites: HashMap<Principal, GroupInvite>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Default for GroupWithMembers {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            website: Default::default(),
            location: Default::default(),
            privacy: Default::default(),
            owner: Principal::anonymous(),
            created_by: Principal::anonymous(),
            matrix_space_id: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            tags: Default::default(),
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: Default::default(),
            notification_id: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            privacy_gated_type_amount: Default::default(),
            special_members: Default::default(),
            members: Default::default(),
            invites: Default::default(),
        }
    }
}

impl GroupWithMembers {
    pub fn from(group: PostGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            owner: caller(),
            created_by: caller(),
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: false,
            notification_id: None,
            updated_on: time(),
            created_on: time(),
            privacy_gated_type_amount: group.privacy_gated_type_amount,
            special_members: HashMap::default(),
            members: Default::default(),
            invites: Default::default(),
        }
    }

    pub fn update(&mut self, group: UpdateGroup) {
        self.name = group.name;
        self.description = group.description;
        self.website = group.website;
        self.location = group.location;
        self.privacy = group.privacy;
        self.image = group.image;
        self.banner_image = group.banner_image;
        self.tags = group.tags;
        self.privacy_gated_type_amount = group.privacy_gated_type_amount;
        self.updated_on = time();
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.members = HashMap::from_iter([(caller(), GroupMember::default().set_owner_role())]);
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_members(&self) -> Vec<Principal> {
        self.members.keys().cloned().collect()
    }

    pub fn remove_member(&mut self, member: Principal) {
        self.members.remove(&member);
    }

    pub fn add_member(&mut self, member: Principal) {
        self.members.insert(member, GroupMember::default());
    }

    pub fn set_member_role(&mut self, member: Principal, role: String) {
        if let Some(member) = self.members.get_mut(&member) {
            member.set_role(role);
        }
    }

    pub fn get_invites(&self) -> Vec<Principal> {
        self.invites.keys().cloned().collect()
    }

    pub fn add_invite(
        &mut self,
        member: Principal,
        invite_type: InviteType,
        notification_id: Option<u64>,
    ) {
        self.invites.insert(
            member,
            GroupInvite {
                notification_id,
                invite_type,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn remove_invite(&mut self, member: Principal) {
        self.invites.remove(&member);
    }

    pub fn convert_invite_to_member(&mut self, principal: Principal) {
        if self.invites.remove(&principal).is_some() {
            self.members.insert(principal, GroupMember::default());
        }
    }

    pub fn get_roles(&self) -> Vec<Role> {
        // set the default protected roles
        let mut roles = default_roles();

        // append the custom roles stored on the group
        roles.append(&mut self.roles.clone());
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
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }

    pub fn add_special_member(&mut self, member: Principal, relation: RelationType) {
        self.special_members.insert(member, relation.to_string());
    }

    pub fn remove_special_member_from_group(&mut self, member: Principal) {
        self.special_members.remove(&member);
    }

    pub fn is_banned_member(&self, member: Principal) -> bool {
        self.special_members
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
    pub privacy: Privacy,
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
    pub privacy: Privacy,
    pub image: Asset,
    pub privacy_gated_type_amount: Option<u64>,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupMember {
    pub roles: Vec<String>,
    pub updated_at: u64,
    pub created_at: u64,
}

impl Default for GroupMember {
    fn default() -> Self {
        Self {
            roles: vec![MEMBER_ROLE.into()],
            updated_at: time(),
            created_at: time(),
        }
    }
}

impl GroupMember {
    pub fn set_owner_role(&mut self) -> Self {
        Self {
            roles: vec![(OWNER_ROLE.into())],
            updated_at: time(),
            created_at: time(),
        }
    }

    pub fn set_role(&mut self, role: String) {
        if ![ADMIN_ROLE, MODERATOR_ROLE, MEMBER_ROLE].contains(&role.as_str()) {
            return;
        }
        self.roles = vec![role];
        self.updated_at = time();
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct GroupInvite {
    pub notification_id: Option<u64>,
    pub invite_type: InviteType,
    pub updated_at: u64,
    pub created_at: u64,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
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
    pub fn new(
        id: u64,
        group: GroupWithMembers,
        boosted: Option<Boost>,
        events_count: u64,
        members_count: u64,
    ) -> Self {
        let mut roles = default_roles();
        roles.append(&mut group.roles.clone());
        Self {
            id,
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            created_by: group.created_by,
            owner: group.owner,
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            roles,
            wallets: group.wallets.into_iter().collect(),
            is_deleted: group.is_deleted,
            privacy_gated_type_amount: group.privacy_gated_type_amount,
            boosted,
            updated_on: group.updated_on,
            created_on: group.created_on,
            events_count,
            members_count,
        }
    }

    pub fn from_result(
        group_result: Result<(u64, GroupWithMembers), ApiError>,
        boosted: Option<Boost>,
        events_count: u64,
        members_count: u64,
    ) -> Result<Self, ApiError> {
        match group_result {
            Err(err) => Err(err),
            Ok((id, group)) => Ok(Self::new(id, group, boosted, events_count, members_count)),
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
            Name(Asc) => {
                groups.sort_by(|(_, a), (_, b)| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            }
            Name(Desc) => {
                groups.sort_by(|(_, a), (_, b)| b.name.to_lowercase().cmp(&a.name.to_lowercase()))
            }
            CreatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.created_on.cmp(&b.created_on)),
            CreatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.created_on.cmp(&a.created_on)),
            UpdatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.updated_on.cmp(&b.updated_on)),
            UpdatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.updated_on.cmp(&a.updated_on)),
            MemberCount(Asc) => {
                groups.sort_by(|(_, a), (_, b)| a.members.len().cmp(&b.members.len()))
            }
            MemberCount(Desc) => {
                groups.sort_by(|(_, a), (_, b)| b.members.len().cmp(&a.members.len()))
            }
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
            Name(name) => group.name.to_lowercase().contains(&name.to_lowercase()),
            Owner(owner) => group.owner == *owner,
            Ids(ids) => ids.contains(id),
            Tag(tag) => group.tags.contains(tag),
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
