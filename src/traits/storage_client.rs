use candid::Principal;

use crate::{helpers::icp::ic_call, CanisterResult};

pub trait StorageClient<K, V, F>: Send + Sync
where
    K: candid::CandidType + for<'a> candid::Deserialize<'a>,
    V: candid::CandidType + for<'a> candid::Deserialize<'a>,
    F: candid::CandidType + Clone,
{
    fn canister(&self) -> Principal;

    fn get(
        &self,
        id: K,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        ic_call(self.canister(), "get", (id,))
    }

    fn get_many(
        &self,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        ic_call(self.canister(), "get_many", (keys,))
    }

    fn get_all(
        &self,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        ic_call(self.canister(), "get_all", ())
    }

    fn find(
        &self,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<Option<(K, V)>>> + Sync + Send {
        ic_call(self.canister(), "find", (filters,))
    }

    fn filter(
        &self,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        ic_call(self.canister(), "filter", (filters,))
    }

    fn insert(
        &self,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        ic_call(self.canister(), "insert", (key, value))
    }

    fn update(
        &self,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        ic_call(self.canister(), "update", (key, value))
    }

    fn remove(
        &self,
        key: K,
    ) -> impl std::future::Future<Output = CanisterResult<bool>> + Sync + Send {
        ic_call(self.canister(), "remove", (key,))
    }

    fn remove_many(
        &self,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<()>> + Sync + Send {
        ic_call(self.canister(), "remove_many", (keys,))
    }
}
