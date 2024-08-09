use std::{cell::RefCell, thread::LocalKey};

use ic_stable_structures::{Cell, Storable};

use crate::{api_error::ApiError, CanisterResult, Memory};

pub type CellStorageRef<V> = RefCell<Cell<Option<V>, Memory>>;
pub type StaticCellStorageRef<V> = &'static LocalKey<CellStorageRef<V>>;

pub trait CellStorage<V: Storable + Clone + 'static> {
    fn name(&self) -> String;
    fn storage(&self) -> StaticCellStorageRef<V>;

    fn get(&self) -> CanisterResult<V> {
        self.storage()
            .with(|data| data.borrow().get().clone())
            .ok_or_else(|| {
                ApiError::unexpected()
                    .add_message(format!("Failed to get {}, not initialized", self.name()))
            })
    }

    fn set(&self, value: V) -> CanisterResult<V> {
        self.storage()
            .with(|data| data.borrow_mut().set(Some(value.clone())))
            .map_err(|_| {
                ApiError::unexpected().add_message(format!("Failed to set {}", self.name()))
            })?;
        Ok(value)
    }

    fn is_empty(&self) -> bool {
        self.storage().with(|data| data.borrow().get().is_none())
    }
}
