use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use serde::Serialize;

use crate::misc::role_misc::{ADMIN_ROLE, MEMBER_ROLE, MODERATOR_ROLE, OWNER_ROLE};

use super::invite_type::InviteType;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Join {
    pub roles: Vec<String>,
    pub updated_at: u64,
    pub created_at: u64,
}

impl Default for Join {
    fn default() -> Self {
        Self {
            roles: vec![MEMBER_ROLE.into()],
            updated_at: time(),
            created_at: time(),
        }
    }
}

impl Join {
    pub fn set_owner_role(&mut self) -> Self {
        Self {
            roles: vec![(OWNER_ROLE.into())],
            updated_at: time(),
            created_at: time(),
        }
    }

    pub fn has_owner_role(&self) -> bool {
        self.roles.contains(&OWNER_ROLE.into())
    }

    pub fn set_admin_role(&mut self) -> Self {
        Self {
            roles: vec![(ADMIN_ROLE.into())],
            updated_at: time(),
            created_at: time(),
        }
    }

    pub fn has_admin_role(&self) -> bool {
        self.roles.contains(&ADMIN_ROLE.into())
    }

    pub fn set_moderator_role(&mut self) -> Self {
        Self {
            roles: vec![(MODERATOR_ROLE.into())],
            updated_at: time(),
            created_at: time(),
        }
    }

    pub fn has_moderator_role(&self) -> bool {
        self.roles.contains(&MODERATOR_ROLE.into())
    }

    pub fn set_member_role(&mut self) -> Self {
        Self {
            roles: vec![(MEMBER_ROLE.into())],
            updated_at: time(),
            created_at: time(),
        }
    }

    pub fn has_member_role(&self) -> bool {
        self.roles.contains(&MEMBER_ROLE.into())
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
pub struct Invite {
    pub notification_id: Option<u64>,
    pub invite_type: InviteType,
    pub updated_at: u64,
    pub created_at: u64,
}

impl Invite {
    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }
}
