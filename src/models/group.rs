use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    misc::role_misc::default_roles,
    models::{asset::Asset, location::Location, privacy::PrivacyType, role::Role},
};

use super::{
    group_with_members::{PostGroup, UpdateGroup},
    old_member::{InviteMemberResponse, JoinedMemberResponse},
    permission::Permission,
    relation_type::RelationType,
};

impl_storable_for!(Group);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: PrivacyType,
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
    pub updated_on: u64,
    pub created_on: u64,
}

impl Default for Group {
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
        }
    }
}

impl Group {
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
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_roles(&self) -> Vec<Role> {
        // set the default roles
        let mut roles = self.roles.clone();

        // append the custom roles stored on the group
        roles.append(&mut default_roles());
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

pub type GroupEntry = (u64, Group);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupCallerData {
    pub joined: Option<JoinedMemberResponse>,
    pub invite: Option<InviteMemberResponse>,
    pub is_starred: bool,
    pub is_pinned: bool,
}

impl GroupCallerData {
    pub fn new(
        joined: Option<JoinedMemberResponse>,
        invite: Option<InviteMemberResponse>,
        is_starred: bool,
        is_pinned: bool,
    ) -> Self {
        Self {
            joined,
            invite,
            is_starred,
            is_pinned,
        }
    }
}
