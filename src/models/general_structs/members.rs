use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::{
    member::{Invite, Join},
    role::Role,
};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug, Default)]
pub struct Members {
    pub members: HashMap<Principal, Join>,
    pub invites: HashMap<Principal, Invite>,
    pub special_members: HashMap<Principal, String>,
    pub roles: Vec<Role>,
}

impl Members {
    pub fn new_with_owner(owner: Principal) -> Self {
        let mut members = HashMap::new();
        members.insert(owner, Join::default().set_owner_role());
        Self {
            members,
            invites: Default::default(),
            special_members: Default::default(),
            roles: Default::default(),
        }
    }

    pub fn set_owner(&mut self, new_owner: Principal) {
        if let Some((_, join)) = self
            .members
            .iter_mut()
            .find(|(_, join)| join.has_owner_role())
        {
            join.set_member_role();
        }
        if let Some(new_owner) = self.members.get_mut(&new_owner) {
            new_owner.set_owner_role();
        }
    }

    pub fn exists(&mut self, member: Principal) -> bool {
        self.members.contains_key(&member)
    }
}
