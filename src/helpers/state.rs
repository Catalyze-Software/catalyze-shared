use std::{cell::RefCell, fmt::Display, thread::LocalKey};

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, Storable,
};

use crate::{CellStorageRef, MemoryManagerStorage};

pub fn init_memory_manager() -> MemoryManagerStorage {
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()))
}

pub fn init_cell<S: Display, V: 'static + Clone + Storable>(
    memory_manager: &'static LocalKey<MemoryManagerStorage>,
    name: S,
    id: MemoryId,
) -> CellStorageRef<V> {
    RefCell::new(
        Cell::init(memory_manager.with(|p| p.borrow().get(id)), None)
            .unwrap_or_else(|_| panic!("Failed to initialize {name} cell")),
    )
}
