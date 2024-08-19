use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::{
    event_with_attendees::EventWithAttendeesEntry, group_with_members::GroupWithMembersEntry,
    profile_with_refs::ProfileWithRefsEntry,
};

#[derive(
    CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum Subject {
    #[default]
    None,
    Group(u64),
    Event(u64),
    Profile(Principal),
    Member(Principal),
    Attendee(Principal),
}

impl Subject {
    pub fn get_type(&self) -> SubjectType {
        match self {
            Subject::None => SubjectType::None,
            Subject::Group(_) => SubjectType::Group,
            Subject::Event(_) => SubjectType::Event,
            Subject::Profile(_) => SubjectType::Profile,
            Subject::Member(_) => SubjectType::Member,
            Subject::Attendee(_) => SubjectType::Attendee,
        }
    }

    pub fn get_id(&self) -> &u64 {
        match self {
            Subject::Group(id) => id,
            Subject::Event(id) => id,
            _ => &0,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubjectType {
    None,
    Group,
    Event,
    Profile,
    Member,
    Attendee,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SubjectResponse {
    None,
    Group(Option<GroupWithMembersEntry>),
    Event(Option<EventWithAttendeesEntry>),
    Profile(Option<ProfileWithRefsEntry>),
    Member(Option<(Principal, Vec<u64>)>),
    Attendee(Option<(Principal, Vec<u64>)>),
}
