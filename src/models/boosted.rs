use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use candid::{Decode, Encode};

use crate::{impl_storable_for, Filter, Sorter};

use super::{
    date_range::DateRange,
    sort_direction::SortDirection,
    subject::{Subject, SubjectType},
};

impl_storable_for!(Boost);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Boost {
    pub subject: Subject,
    pub seconds: u64,
    pub owner: Principal,
    pub blockheight: u64,
    pub notification_id: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Boost {
    pub fn new(subject: Subject, seconds: u64, owner: Principal, blockheight: u64) -> Self {
        Self {
            subject,
            seconds,
            created_at: time(),
            updated_at: time(),
            owner,
            notification_id: None,
            blockheight,
        }
    }

    pub fn update(&mut self, seconds: u64) {
        self.seconds = seconds;
        self.updated_at = time();
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }
}

pub type BoostedEntry = (u64, Boost);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum BoostedSort {
    CreatedAt(SortDirection),
    UpdatedAt(SortDirection),
}

impl Default for BoostedSort {
    fn default() -> Self {
        BoostedSort::CreatedAt(SortDirection::default())
    }
}

impl Sorter<u64, Boost> for BoostedSort {
    fn sort(&self, boosteds: Vec<BoostedEntry>) -> Vec<BoostedEntry> {
        let mut boosteds = boosteds;

        use BoostedSort::*;
        use SortDirection::*;
        match self {
            CreatedAt(Asc) => boosteds.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at)),
            CreatedAt(Desc) => boosteds.sort_by(|a, b| b.1.created_at.cmp(&a.1.created_at)),
            UpdatedAt(Asc) => boosteds.sort_by(|a, b| a.1.updated_at.cmp(&b.1.updated_at)),
            UpdatedAt(Desc) => boosteds.sort_by(|a, b| b.1.updated_at.cmp(&a.1.updated_at)),
        }
        boosteds
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub enum BoostedFilter {
    #[default]
    None,
    Ids(Vec<u64>),
    Subject(Subject),
    SubjectType(SubjectType),
    Owner(Principal),
    UpdatedAt(DateRange),
    CreatedAt(DateRange),
}

impl Filter<u64, Boost> for BoostedFilter {
    fn matches(&self, id: &u64, boosted: &Boost) -> bool {
        use BoostedFilter::*;
        match self {
            None => true,
            Ids(ids) => ids.contains(id),
            Subject(subject) => *subject == boosted.subject,
            SubjectType(subject_type) => *subject_type == boosted.subject.get_type(),
            Owner(owner) => *owner == boosted.owner,
            UpdatedAt(date) => date.is_within(boosted.updated_at),
            CreatedAt(date) => date.is_within(boosted.created_at),
        }
    }
}

impl From<BoostedFilter> for Vec<BoostedFilter> {
    fn from(val: BoostedFilter) -> Self {
        vec![val]
    }
}
