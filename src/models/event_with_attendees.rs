use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::PrivacyType,
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
    relation_type::RelationType,
};

impl_storable_for!(EventWithAttendees);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct EventWithAttendees {
    pub metadata: Metadata,
    pub dates: Vec<DateRange>,
    pub privacy: Privacy,
    pub group_id: Option<u64>,
    pub created_by: Principal,
    pub owner: Principal,
    pub references: References,
    pub is_canceled: Option<String>,
    pub is_deleted: bool,
    pub attendees: Members,
    pub updated_on: u64,
    pub created_on: u64,
}

impl From<PostEvent> for EventWithAttendees {
    fn from(post_event: PostEvent) -> Self {
        Self {
            metadata: Metadata {
                name: post_event.name.clone(),
                description: post_event.description.clone(),
                image: post_event.image.clone(),
                banner_image: post_event.banner_image.clone(),
                website: post_event.website.clone(),
                location: post_event.location.clone(),
            },
            dates: vec![post_event.date],
            privacy: Privacy {
                privacy_type: post_event.privacy,
                privacy_gated_type_amount: None,
            },
            group_id: post_event.group_id,
            created_by: caller(),
            owner: caller(),
            references: References::default(),
            is_canceled: None,
            is_deleted: false,
            attendees: Members::new_with_owner(caller()),
            updated_on: time(),
            created_on: time(),
        }
    }
}

impl EventWithAttendees {
    pub fn update(&mut self, event: UpdateEvent) -> Self {
        self.metadata.name = event.name;
        self.metadata.description = event.description;
        self.metadata.website = event.website;
        self.metadata.location = event.location;
        self.metadata.image = event.image;
        self.metadata.banner_image = event.banner_image;
        self.privacy.privacy_type = event.privacy;
        self.references.tags = event.tags;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_total_date_range(&self) -> DateRange {
        let mut start_dates = self.dates.clone();
        start_dates.sort_by_key(|range| range.start_date());

        let mut end_dates = self.dates.clone();
        end_dates.sort_by_key(|range| range.end_date());

        let start_date = start_dates
            .first()
            .map(|date| date.start_date())
            .unwrap_or(time());

        let end_date = end_dates
            .last()
            .map(|date| date.end_date())
            .unwrap_or(time());
        DateRange::new(start_date, end_date)
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.attendees.set_owner(owner);
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_members(&self) -> Vec<Principal> {
        self.attendees.members.keys().cloned().collect()
    }

    pub fn remove_attendee(&mut self, member: Principal) {
        self.attendees.members.remove(&member);
    }

    pub fn add_attendee(&mut self, member: Principal) {
        self.attendees.members.insert(member, Join::default());
    }

    pub fn set_attendee_role(&mut self, member: Principal, role: String) {
        if let Some(member) = self.attendees.members.get_mut(&member) {
            member.set_role(role);
        }
    }

    pub fn get_invites(&self) -> Vec<Principal> {
        self.attendees.invites.keys().cloned().collect()
    }

    pub fn add_invite(
        &mut self,
        member: Principal,
        invite_type: InviteType,
        notification_id: Option<u64>,
    ) {
        self.attendees.invites.insert(
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
        self.attendees.invites.remove(&member);
    }

    pub fn convert_invite_to_attendee(&mut self, principal: Principal) {
        if self.attendees.invites.remove(&principal).is_some() {
            self.attendees.members.insert(principal, Join::default());
        }
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.references.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.references.notification_id = None;
    }

    pub fn add_special_attendees(&mut self, member: Principal, relation: RelationType) {
        self.attendees
            .special_members
            .insert(member, relation.to_string());
    }

    pub fn remove_special_attendee(&mut self, member: Principal) {
        self.attendees.special_members.remove(&member);
    }

    pub fn is_banned_member(&self, member: Principal) -> bool {
        self.attendees
            .special_members
            .get(&member)
            .map(|relation| relation == &RelationType::Blocked.to_string())
            .unwrap_or_default()
    }

    pub fn is_attendee(&self, attendee: Principal) -> bool {
        self.attendees.exists(attendee)
    }
}

impl Default for EventWithAttendees {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            dates: vec![DateRange::default()],
            privacy: Privacy::default(),
            group_id: None,
            created_by: Principal::anonymous(),
            owner: Principal::anonymous(),
            references: References::default(),
            is_canceled: None,
            is_deleted: false,
            attendees: Members::default(),
            updated_on: time(),
            created_on: time(),
        }
    }
}

pub type EventEntry = (u64, EventWithAttendees);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PostEvent {
    name: String,
    description: String,
    date: DateRange,
    privacy: PrivacyType,
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
    pub privacy: PrivacyType,
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
            StartDate(Asc) => events.sort_by(|a, b| {
                a.1.get_total_date_range()
                    .start_date()
                    .cmp(&b.1.get_total_date_range().start_date())
            }),
            StartDate(Desc) => events.sort_by(|a, b| {
                b.1.get_total_date_range()
                    .start_date()
                    .cmp(&a.1.get_total_date_range().start_date())
            }),
            EndDate(Asc) => events.sort_by(|a, b| {
                a.1.get_total_date_range()
                    .end_date()
                    .cmp(&b.1.get_total_date_range().end_date())
            }),
            EndDate(Desc) => events.sort_by(|a, b| {
                b.1.get_total_date_range()
                    .end_date()
                    .cmp(&a.1.get_total_date_range().end_date())
            }),
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
            Name(name) => event
                .metadata
                .name
                .to_lowercase()
                .contains(&name.to_lowercase()),
            StartDate(date) => {
                if date.is_within(time()) {
                    return true;
                }

                date.is_within(event.get_total_date_range().start_date())
            }
            EndDate(date) => date.is_within(event.get_total_date_range().end_date()),
            Owner(owner) => *owner == event.owner,
            Groups(groups) => groups.contains(&event.group_id),
            Ids(ids) => ids.contains(id),
            Tag(tag) => event.references.tags.contains(tag),
            IsCanceled(is_canceled) => event.is_canceled.is_some() == *is_canceled,
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
    pub privacy: PrivacyType,
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
            name: event.metadata.name.clone(),
            description: event.metadata.description.clone(),
            date: event.get_total_date_range().clone(),
            privacy: event.privacy.privacy_type,
            created_by: event.created_by,
            owner: event.owner,
            website: event.metadata.website,
            location: event.metadata.location,
            image: event.metadata.image,
            banner_image: event.metadata.banner_image,
            is_canceled: (
                event.is_canceled.is_some(),
                event.is_canceled.unwrap_or_default(),
            ),
            is_deleted: event.is_deleted,
            tags: event.references.tags,
            metadata: None,
            updated_on: event.updated_on,
            created_on: event.created_on,
            group_id: event.group_id,
            attendee_count: event.attendees.members.len() as u64,
            invite_count: event.attendees.invites.len() as u64,
            boosted,
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
