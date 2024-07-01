use std::{cell::RefCell, thread::LocalKey};

use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type StorageRef<K, V> = RefCell<StableBTreeMap<K, V, Memory>>;
pub type StaticStorageRef<K, V> = &'static LocalKey<StorageRef<K, V>>;
pub type MemoryManagerStorage = RefCell<MemoryManager<DefaultMemoryImpl>>;
