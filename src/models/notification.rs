use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::{Deserialize, Serialize};

use candid::{Decode, Encode};

use crate::{impl_storable_for, Filter, Sorter};

use super::{
    attendee::{InviteAttendeeResponse, JoinedAttendeeResponse},
    friend_request::FriendRequestResponse,
    old_member::{InviteMemberResponse, JoinedMemberResponse},
    sort_direction::SortDirection,
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::UserNotificationData,
};

impl_storable_for!(Notification);
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Notification {
    pub notification_type: NotificationType,
    // used on the frontend to determine if the notification is actionable
    // this value changes based on the action the user takes
    pub is_actionable: bool,
    pub processed_by: Option<Principal>,
    pub is_accepted: Option<bool>,
    // additional data for the notification that the frontend can utilize
    pub metadata: Option<String>,
    pub sender: Principal,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Notification {
    pub fn new(notification_type: NotificationType, is_actionable: bool) -> Self {
        Self {
            notification_type,
            is_actionable,
            is_accepted: None,
            metadata: None,
            processed_by: None,
            sender: caller(),
            created_at: time(),
            updated_at: time(),
        }
    }

    pub fn mark_as_accepted(&mut self, is_accepted: bool, notification_type: NotificationType) {
        self.is_accepted = Some(is_accepted);
        self.is_actionable = false;
        self.processed_by = Some(caller());
        self.updated_at = time();
        self.notification_type = notification_type;
    }

    pub fn set_metadata(&mut self, metadata: String) {
        self.metadata = Some(metadata);
        self.updated_at = time();
    }

    pub fn set_is_actionable(&mut self, is_actionable: bool) {
        self.is_actionable = is_actionable;
        self.updated_at = time();
    }
}

pub type NotificationEntry = (u64, Notification);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum NotificationSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

impl Default for NotificationSort {
    fn default() -> Self {
        NotificationSort::CreatedOn(SortDirection::Asc)
    }
}

impl Sorter<u64, Notification> for NotificationSort {
    fn sort(&self, notifications: Vec<(u64, Notification)>) -> Vec<(u64, Notification)> {
        let mut notifications: Vec<(u64, Notification)> = notifications.into_iter().collect();
        use NotificationSort::*;
        use SortDirection::*;
        match self {
            CreatedOn(Asc) => {
                notifications.sort_by(|(_, a), (_, b)| a.created_at.cmp(&b.created_at))
            }
            CreatedOn(Desc) => {
                notifications.sort_by(|(_, a), (_, b)| b.created_at.cmp(&a.created_at))
            }
            UpdatedOn(Asc) => {
                notifications.sort_by(|(_, a), (_, b)| a.updated_at.cmp(&b.updated_at))
            }
            UpdatedOn(Desc) => {
                notifications.sort_by(|(_, a), (_, b)| b.updated_at.cmp(&a.updated_at))
            }
        }
        notifications
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub enum NotificationFilter {
    #[default]
    None,
    Ids(Vec<u64>),
    Type(NotificationType),
    Actionable(bool),
    ProcessedBy(Principal),
    Sender(Principal),
}

impl Filter<u64, Notification> for NotificationFilter {
    fn matches(&self, id: &u64, notification: &Notification) -> bool {
        use NotificationFilter::*;
        match self {
            None => true,
            Ids(ids) => ids.contains(id),
            Type(filter) => match filter {
                NotificationType::Relation(_) => {
                    matches!(
                        notification.notification_type,
                        NotificationType::Relation(_)
                    )
                }
                NotificationType::Group(_) => {
                    matches!(notification.notification_type, NotificationType::Group(_))
                }
                NotificationType::Event(_) => {
                    matches!(notification.notification_type, NotificationType::Event(_))
                }
                NotificationType::Transaction(_) => {
                    matches!(
                        notification.notification_type,
                        NotificationType::Transaction(_)
                    )
                }
                NotificationType::Multisig(_) => {
                    matches!(
                        notification.notification_type,
                        NotificationType::Multisig(_)
                    )
                }
            },
            Actionable(actionable) => notification.is_actionable == *actionable,
            ProcessedBy(processed_by) => {
                matches!(notification.processed_by, Some(principal) if principal == *processed_by)
            }
            Sender(sender) => notification.sender == *sender,
        }
    }
}

impl From<NotificationFilter> for Vec<NotificationFilter> {
    fn from(val: NotificationFilter) -> Self {
        vec![val]
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum NotificationType {
    Relation(RelationNotificationType),
    Group(GroupNotificationType),
    Event(EventNotificationType),
    Transaction(TransactionNotificationType),
    Multisig(MultisigNotificationType),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransactionNotificationType {
    SingleTransaction(TransactionData),
    TransactionsComplete(TransactionCompleteData),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum MultisigNotificationType {
    WhitelistNotice((Principal, u64)),
    NewProposal((Principal, u64, u64)),
    ProposalAccept((Principal, u64, u64)),
    ProposalDecline((Principal, u64, u64)),
    ProposalStatusUpdate((Principal, u64, u64)),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum RelationNotificationType {
    FriendRequest(FriendRequestResponse),
    FriendRequestAccept(FriendRequestResponse),
    FriendRequestDecline(FriendRequestResponse),

    FriendRequestRemove(u64),   // friend_request_id
    FriendRemove(Principal),    // user principal
    BlockUser(Principal),       // user principal
    FriendRequestReminder(u64), // friend_request_id
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum GroupNotificationType {
    // user wants to join the group
    JoinGroupUserRequest(InviteMemberResponse),
    JoinGroupUserRequestAccept(InviteMemberResponse),
    JoinGroupUserRequestDecline(InviteMemberResponse),
    // group wants a user to join
    JoinGroupOwnerRequest(InviteMemberResponse),
    JoinGroupOwnerRequestAccept(InviteMemberResponse),
    JoinGroupOwnerRequestDecline(InviteMemberResponse),

    RoleAssignByOwner(JoinedMemberResponse),
    RemoveInviteByOwner(InviteMemberResponse),
    RemoveMemberByOwner(JoinedMemberResponse),

    UserLeaveGroup(u64),
    UserJoinGroup(u64),
    GroupReminder(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventNotificationType {
    // user wants to join the event
    JoinEventUserRequest(InviteAttendeeResponse),
    JoinEventUserRequestAccept(InviteAttendeeResponse),
    JoinEventUserRequestDecline(InviteAttendeeResponse),

    // Event wants a user to join
    JoinEventOwnerRequest(InviteAttendeeResponse),
    JoinEventOwnerRequestAccept(InviteAttendeeResponse),
    JoinEventOwnerRequestDecline(InviteAttendeeResponse),

    RoleAssignByOwner(JoinedAttendeeResponse),
    RemoveInviteByOwner(InviteAttendeeResponse),
    RemoveAttendeeByOwner(JoinedAttendeeResponse),

    UserJoinEvent((u64, u64)),
    UserLeaveEvent((u64, u64)),
    EventReminder(u64),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct NotificationResponse {
    pub id: Option<u64>,
    pub notification: Notification,
    pub user_data: Option<UserNotificationData>,
}

impl NotificationResponse {
    pub fn new(
        id: Option<u64>,
        notification: Notification,
        user_data: Option<UserNotificationData>,
    ) -> Self {
        Self {
            id,
            notification,
            user_data,
        }
    }
}
