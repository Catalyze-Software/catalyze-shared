use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::{impl_storable_for, Filter};

use super::sort_direction::SortDirection;

impl_storable_for!(FriendRequest);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct FriendRequest {
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub notification_id: Option<u64>,
    pub created_at: u64,
}

impl FriendRequest {
    pub fn new(requested_by: Principal, to: Principal, message: String) -> Self {
        Self {
            requested_by,
            message,
            to,
            notification_id: None,
            created_at: time(),
        }
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }
}

pub type FriendRequestEntry = (u64, FriendRequest);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum FriendRequestSort {
    CreatedAt(SortDirection),
}

impl Default for FriendRequestSort {
    fn default() -> Self {
        FriendRequestSort::CreatedAt(SortDirection::Asc)
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum FriendRequestFilter {
    Requestor(Principal),
    Recipient(Principal),
}

impl Filter<u64, FriendRequest> for FriendRequestFilter {
    fn matches(&self, _key: &u64, request: &FriendRequest) -> bool {
        match self {
            FriendRequestFilter::Requestor(requestor) => request.requested_by == *requestor,
            FriendRequestFilter::Recipient(recipient) => request.to == *recipient,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct FriendRequestResponse {
    pub id: u64,
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}

impl FriendRequestResponse {
    pub fn new(id: u64, friend_request: FriendRequest) -> Self {
        Self {
            id,
            requested_by: friend_request.requested_by,
            message: friend_request.message,
            to: friend_request.to,
            created_at: friend_request.created_at,
        }
    }
}
