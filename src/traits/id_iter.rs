use std::fmt::Display;

use ic_stable_structures::Storable;

use crate::{CanisterResult, StaticStorageRef};

pub trait IDIterStorage<Kind: Clone + Display, T: 'static + Storable>: Clone + Sized {
    fn storage(&self) -> StaticStorageRef<String, u64>;

    fn storage_by_kind(&self, kind: Kind) -> StaticStorageRef<u64, T>;

    fn name(&self) -> String {
        "id_iter_storage".to_owned()
    }

    fn get(&self, kind: Kind) -> Option<u64> {
        self.storage()
            .with(|data| data.borrow().get(&kind.to_string()))
    }

    fn get_all(&self) -> Vec<(String, u64)> {
        self.storage().with(|data| data.borrow().iter().collect())
    }

    fn next(&self, kind: Kind) -> CanisterResult<u64> {
        let id = self
            .get(kind.clone())
            .unwrap_or_else(|| self.get_last_key(kind.clone()))
            + 1;

        self.storage().with(|data| {
            data.borrow_mut().insert(kind.to_string(), id);
            Ok(id)
        })
    }

    fn get_last_key(&self, kind: Kind) -> u64 {
        self.storage_by_kind(kind)
            .with(|data| data.borrow().last_key_value().map(|(k, _)| k).unwrap_or(1))
    }
}
