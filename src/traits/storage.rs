use std::fmt::Display;

use ic_stable_structures::{memory_manager::MemoryId, StableBTreeMap, Storable};

use crate::{api_error::ApiError, CanisterResult, StaticMemoryManagerStorage, StaticStorageRef};

pub trait Storage<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone> {
    fn name(&self) -> String;
    fn storage(&self) -> StaticStorageRef<Key, Value>;
    fn memory_id(&self) -> MemoryId;
    fn memory_manager(&self) -> StaticMemoryManagerStorage;
}

pub trait StorageQueryable<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone>:
    Storage<Key, Value>
{
    fn size(&self) -> u64 {
        self.storage().with(|data| data.borrow().len())
    }

    fn get(&self, key: Key) -> CanisterResult<(Key, Value)> {
        self.storage().with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(
                    ApiError::not_found()
                        .add_method_name("get")
                        .add_info(self.name()),
                )
                .map(|value| (key, value))
        })
    }

    fn get_opt(&self, key: Key) -> Option<(Key, Value)> {
        self.storage()
            .with(|data| data.borrow().get(&key).map(|value| (key, value)))
    }

    fn get_many(&self, keys: Vec<Key>) -> Vec<(Key, Value)> {
        self.storage().with(|data| {
            let mut entities = Vec::new();
            for key in keys {
                if let Some(value) = data.borrow().get(&key) {
                    entities.push((key, value));
                }
            }
            entities
        })
    }

    fn get_all(&self) -> Vec<(Key, Value)> {
        self.storage().with(|data| data.borrow().iter().collect())
    }

    fn find<F>(&self, filter: F) -> Option<(Key, Value)>
    where
        F: Fn(&Key, &Value) -> bool,
    {
        self.storage()
            .with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    fn filter<F>(&self, filter: F) -> Vec<(Key, Value)>
    where
        F: Fn(&Key, &Value) -> bool,
    {
        self.storage().with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    fn contains_key(&self, key: Key) -> bool {
        self.storage().with(|data| data.borrow().contains_key(&key))
    }
}

pub trait StorageUpdateable<
    Key: 'static + Storable + Ord + Clone,
    Value: 'static + Storable + Clone,
>: StorageQueryable<Key, Value>
{
    fn update(&self, key: Key, value: Value) -> CanisterResult<(Key, Value)> {
        self.storage().with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(self.name())
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }

    fn upsert(&self, key: Key, value: Value) -> CanisterResult<(Key, Value)> {
        self.storage().with(|data| {
            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }

    fn remove(&self, key: Key) -> CanisterResult<()> {
        self.storage().with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("remove")
                    .add_info(self.name())
                    .add_message("Key does not exist"));
            }
            data.borrow_mut().remove(&key);
            Ok(())
        })
    }

    fn remove_many(&self, keys: Vec<Key>) {
        self.storage().with(|data| {
            for key in keys {
                data.borrow_mut().remove(&key);
            }
        })
    }

    /// Clear all entities
    fn clear(&self) {
        self.storage().with(|n| {
            n.replace(StableBTreeMap::new(
                self.memory_manager()
                    .with(|m| m.borrow().get(self.memory_id())),
            ))
        });
    }
}

pub trait StorageInsertable<Value: 'static + Storable + Clone>:
    StorageUpdateable<u64, Value>
{
    fn insert<F>(&self, next_key_fn: F, value: Value) -> CanisterResult<(u64, Value)>
    where
        F: Fn(String) -> CanisterResult<u64>,
    {
        self.storage().with(|data| {
            let key = next_key_fn(self.name())?;

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(self.name())
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }
}

pub trait StorageInsertableByKey<
    Key: 'static + Storable + Ord + Clone,
    Value: 'static + Storable + Clone,
>: StorageUpdateable<Key, Value>
{
    fn insert_by_key(&self, key: Key, value: Value) -> CanisterResult<(Key, Value)> {
        self.storage().with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(self.name())
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }
}

#[derive(Clone)]
pub struct GenericStorage<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone>
{
    name: String,
    storage: StaticStorageRef<Key, Value>,
    memory_id: MemoryId,
    memory_manager: StaticMemoryManagerStorage,
}

impl<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone>
    GenericStorage<Key, Value>
{
    pub fn new<S: Display>(
        name: S,
        storage: StaticStorageRef<Key, Value>,
        memory_id: MemoryId,
        memory_manager: StaticMemoryManagerStorage,
    ) -> Self {
        Self {
            name: name.to_string(),
            storage,
            memory_id,
            memory_manager,
        }
    }
}

impl<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone> Storage<Key, Value>
    for GenericStorage<Key, Value>
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<Key, Value> {
        self.storage
    }

    fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    fn memory_manager(&self) -> StaticMemoryManagerStorage {
        self.memory_manager
    }
}

impl<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone>
    StorageQueryable<Key, Value> for GenericStorage<Key, Value>
{
}

impl<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone>
    StorageUpdateable<Key, Value> for GenericStorage<Key, Value>
{
}

impl<Value: 'static + Storable + Clone> StorageInsertable<Value> for GenericStorage<u64, Value> {}

impl<Key: 'static + Storable + Ord + Clone, Value: 'static + Storable + Clone>
    StorageInsertableByKey<Key, Value> for GenericStorage<Key, Value>
{
}
