use candid::Principal;
use group::GroupStorageClient;

use crate::StaticCellStorageRef;

mod group;

pub fn groups(canister_storage: StaticCellStorageRef<Principal>) -> GroupStorageClient {
    GroupStorageClient::new(canister_storage)
}
