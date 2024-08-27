use crate::{
    group_with_members::{GroupFilter, GroupSort, GroupWithMembers},
    StaticCellStorageRef, StorageClient, StorageClientInsertable,
};
use candid::Principal;

#[derive(Clone)]
pub struct GroupStorageClient {
    canister: StaticCellStorageRef<Principal>,
}

impl GroupStorageClient {
    pub fn new(canister: StaticCellStorageRef<Principal>) -> Self {
        Self { canister }
    }
}

impl StorageClient<u64, GroupWithMembers, GroupFilter, GroupSort> for GroupStorageClient {
    fn name(&self) -> String {
        "group".to_string()
    }

    fn storage_canister_id(&self) -> StaticCellStorageRef<Principal> {
        self.canister
    }
}

impl StorageClientInsertable<GroupWithMembers, GroupFilter, GroupSort> for GroupStorageClient {}
