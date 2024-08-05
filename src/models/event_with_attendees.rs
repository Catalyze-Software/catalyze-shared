use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::Privacy,
        sort_direction::SortDirection,
    },
    Filter, Sorter,
};

use super::{
    api_error::ApiError,
    boosted::Boost,
    invite_type::InviteType,
    member::{Invite, Join},
};

impl_storable_for!(EventWithAttendees);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct EventWithAttendees {
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub group_id: Option<u64>,
    pub created_by: Principal,
    pub owner: Principal,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub is_canceled: (bool, String),
    pub is_deleted: bool,
    pub metadata: Option<String>,
    pub attendees: HashMap<Principal, Join>,
    pub invites: HashMap<Principal, Invite>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl EventWithAttendees {
    pub fn match_privacy(&self, privacy: Privacy) -> bool {
        self.privacy == privacy
    }
}

impl From<PostEvent> for EventWithAttendees {
    fn from(post_event: PostEvent) -> Self {
        Self {
            name: post_event.name,
            description: post_event.description,
            date: post_event.date,
            privacy: post_event.privacy,
            group_id: post_event.group_id,
            created_by: caller(),
            owner: caller(),
            website: post_event.website,
            location: post_event.location,
            image: post_event.image,
            banner_image: post_event.banner_image,
            tags: post_event.tags,
            is_canceled: (false, "".to_string()),
            is_deleted: false,
            metadata: post_event.metadata,
            updated_on: time(),
            created_on: time(),
            attendees: Default::default(),
            invites: Default::default(),
        }
    }
}

impl EventWithAttendees {
    pub fn update(&mut self, update_event: UpdateEvent) -> Self {
        self.name = update_event.name;
        self.description = update_event.description;
        self.date = update_event.date;
        self.privacy = update_event.privacy;
        self.website = update_event.website;
        self.location = update_event.location;
        self.image = update_event.image;
        self.banner_image = update_event.banner_image;
        self.tags = update_event.tags;
        self.metadata = update_event.metadata;
        self.updated_on = time();
        self.clone()
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.attendees
            .insert(owner, Join::default().set_owner_role());
        self.updated_on = time();
        self.clone()
    }

    pub fn get_attendee(&self) -> Vec<Principal> {
        self.attendees.keys().cloned().collect()
    }

    pub fn remove_attendee(&mut self, attendee: Principal) {
        self.attendees.remove(&attendee);
    }

    pub fn add_attendee(&mut self, attendee: Principal) {
        self.attendees.insert(attendee, Join::default());
    }

    pub fn set_attendee_role(&mut self, attendee: Principal, role: String) {
        if let Some(attendee) = self.attendees.get_mut(&attendee) {
            attendee.set_role(role);
        }
    }

    pub fn get_invites(&self) -> Vec<Principal> {
        self.invites.keys().cloned().collect()
    }

    pub fn add_invite(
        &mut self,
        attendee: Principal,
        invite_type: InviteType,
        notification_id: Option<u64>,
    ) {
        self.invites.insert(
            attendee,
            Invite {
                notification_id,
                invite_type,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn remove_invite(&mut self, attendee: Principal) {
        self.invites.remove(&attendee);
    }

    pub fn convert_invite_to_attendee(&mut self, principal: Principal) {
        if self.invites.remove(&principal).is_some() {
            self.attendees.insert(principal, Join::default());
        }
    }

    pub fn cancel(&mut self, reason: String) -> Self {
        self.is_canceled = (true, reason);
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn is_from_group(&self, group_id: Option<u64>) -> bool {
        self.group_id == group_id
    }
}

impl Default for EventWithAttendees {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            date: Default::default(),
            privacy: Default::default(),
            group_id: Default::default(),
            created_by: Principal::anonymous(),
            owner: Principal::anonymous(),
            website: Default::default(),
            location: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            tags: Default::default(),
            is_canceled: Default::default(),
            is_deleted: Default::default(),
            attendees: Default::default(),
            invites: Default::default(),
            metadata: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
        }
    }
}

pub type EventEntry = (u64, EventWithAttendees);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PostEvent {
    name: String,
    description: String,
    date: DateRange,
    privacy: Privacy,
    website: String,
    location: Location,
    image: Asset,
    banner_image: Asset,
    group_id: Option<u64>,
    metadata: Option<String>,
    tags: Vec<u32>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UpdateEvent {
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub owner: Principal,
    pub banner_image: Asset,
    pub metadata: Option<String>,
    pub tags: Vec<u32>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
    StartDate(SortDirection),
    EndDate(SortDirection),
}

impl Default for EventSort {
    fn default() -> Self {
        EventSort::CreatedOn(SortDirection::default())
    }
}

impl Sorter<u64, EventWithAttendees> for EventSort {
    fn sort(&self, events: Vec<(u64, EventWithAttendees)>) -> Vec<(u64, EventWithAttendees)> {
        let mut events: Vec<(u64, EventWithAttendees)> = events.into_iter().collect();
        use EventSort::*;
        use SortDirection::*;
        match self {
            CreatedOn(Asc) => events.sort_by(|a, b| a.1.created_on.cmp(&b.1.created_on)),
            CreatedOn(Desc) => events.sort_by(|a, b| b.1.created_on.cmp(&a.1.created_on)),
            UpdatedOn(Asc) => events.sort_by(|a, b| a.1.updated_on.cmp(&b.1.updated_on)),
            UpdatedOn(Desc) => events.sort_by(|a, b| b.1.updated_on.cmp(&a.1.updated_on)),
            StartDate(Asc) => {
                events.sort_by(|a, b| a.1.date.start_date().cmp(&b.1.date.start_date()))
            }
            StartDate(Desc) => {
                events.sort_by(|a, b| b.1.date.start_date().cmp(&a.1.date.start_date()))
            }
            EndDate(Asc) => events.sort_by(|a, b| a.1.date.end_date().cmp(&b.1.date.end_date())),
            EndDate(Desc) => events.sort_by(|a, b| b.1.date.end_date().cmp(&a.1.date.end_date())),
        }
        events
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub enum EventFilter {
    #[default]
    None,
    Name(String),
    StartDate(DateRange),
    EndDate(DateRange),
    Owner(Principal),
    Groups(Vec<Option<u64>>),
    Ids(Vec<u64>),
    Tag(u32),
    IsCanceled(bool),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}

impl Filter<u64, EventWithAttendees> for EventFilter {
    fn matches(&self, id: &u64, event: &EventWithAttendees) -> bool {
        use EventFilter::*;
        match self {
            None => true,
            Name(name) => event.name.to_lowercase().contains(&name.to_lowercase()),
            StartDate(date) => {
                if date.is_within(time()) {
                    return true;
                }

                date.is_within(event.date.start_date())
            }
            EndDate(date) => date.is_within(event.date.end_date()),
            Owner(owner) => *owner == event.owner,
            Groups(groups) => groups.contains(&event.group_id),
            Ids(ids) => ids.contains(id),
            Tag(tag) => event.tags.contains(tag),
            IsCanceled(is_canceled) => event.is_canceled.0 == *is_canceled,
            UpdatedOn(date) => date.is_within(event.updated_on),
            CreatedOn(date) => date.is_within(event.created_on),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EventsCount {
    pub total: u64,
    pub attending: u64,
    pub invited: u64,
    pub starred: u64,
    pub future: u64,
    pub past: u64,
    pub new: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EventResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub created_by: Principal,
    pub owner: Principal,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub banner_image: Asset,
    pub is_canceled: (bool, String),
    pub is_deleted: bool,
    pub tags: Vec<u32>,
    pub metadata: Option<String>,
    pub updated_on: u64,
    pub created_on: u64,
    pub group_id: Option<u64>,
    pub attendee_count: u64,
    pub invite_count: u64,
    pub boosted: Option<Boost>,
}

impl EventResponse {
    pub fn new(id: u64, event: EventWithAttendees, boosted: Option<Boost>) -> Self {
        Self {
            id,
            name: event.name,
            description: event.description,
            date: event.date,
            privacy: event.privacy,
            created_by: event.created_by,
            owner: event.owner,
            website: event.website,
            location: event.location,
            image: event.image,
            banner_image: event.banner_image,
            is_canceled: event.is_canceled,
            is_deleted: event.is_deleted,
            tags: event.tags,
            metadata: event.metadata,
            updated_on: event.updated_on,
            created_on: event.created_on,
            group_id: event.group_id,
            boosted,
            attendee_count: event.attendees.len() as u64,
            invite_count: event.invites.len() as u64,
        }
    }

    pub fn from_result(
        id: u64,
        event: Result<EventWithAttendees, ApiError>,
        boosted: Option<Boost>,
    ) -> Result<Self, ApiError> {
        match event {
            Err(e) => Err(e),
            Ok(event) => Ok(Self::new(id, event, boosted)),
        }
    }
}

impl From<EventFilter> for Vec<EventFilter> {
    fn from(val: EventFilter) -> Self {
        vec![val]
    }
}
