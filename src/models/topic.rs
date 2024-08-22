use std::fmt::{Display, Formatter};

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{str::eq_str, Filter, Sorter};

use super::{api_error::ApiError, sort_direction::SortDirection};

use crate::impl_storable_for;

impl_storable_for!(Topic);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize, Serialize)]
pub enum TopicKind {
    Tag,
    Category,
    Skill,
}

impl Display for TopicKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicKind::Tag => write!(f, "Tag"),
            TopicKind::Category => write!(f, "Category"),
            TopicKind::Skill => write!(f, "Skill"),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Topic {
    pub kind: TopicKind,
    pub value: String,
}

pub type TopicEntry = (u64, Topic);

impl From<((u64, String), TopicKind)> for Topic {
    fn from(((_id, value), kind): ((u64, String), TopicKind)) -> Self {
        Self { kind, value }
    }
}

impl Topic {
    pub fn new(kind: TopicKind, value: String) -> Self {
        Self { kind, value }
    }
}

impl From<Topic> for Result<Topic, ApiError> {
    fn from(val: Topic) -> Self {
        Ok(val)
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum TopicSort {
    ID(SortDirection),
}

impl Default for TopicSort {
    fn default() -> Self {
        TopicSort::ID(SortDirection::Asc)
    }
}

impl Sorter<u64, Topic> for TopicSort {
    fn sort(&self, data: Vec<TopicEntry>) -> Vec<TopicEntry> {
        match self {
            TopicSort::ID(direction) => {
                let mut data = data;
                data.sort_by(|(a, _), (b, _)| match direction {
                    SortDirection::Asc => a.cmp(b),
                    SortDirection::Desc => b.cmp(a),
                });
                data
            }
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum TopicFilter {
    Kind(TopicKind),
    Value(String),
}

impl Filter<u64, Topic> for TopicFilter {
    fn matches(&self, _key: &u64, topic: &Topic) -> bool {
        match self {
            TopicFilter::Kind(kind) => topic.kind == *kind,
            TopicFilter::Value(value) => eq_str(topic.value.clone(), value.clone()),
        }
    }
}

impl From<TopicFilter> for Vec<TopicFilter> {
    fn from(val: TopicFilter) -> Self {
        vec![val]
    }
}
