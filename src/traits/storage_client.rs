use candid::Principal;

use crate::{helpers::ic_call::ic_call, CanisterResult};

pub trait StorageClient<K, V, F>: Send + Sync
where
    K: candid::CandidType + for<'a> candid::Deserialize<'a> + Sync + Send,
    V: candid::CandidType + for<'a> candid::Deserialize<'a> + Sync + Send,
    F: candid::CandidType + Clone + Sync + Send,
{
    fn canister(&self) -> CanisterResult<Principal>;

    fn get(
        &self,
        id: K,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        async move { ic_call(self.canister()?, "get", (id,)).await }
    }

    fn get_many(
        &self,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        async move { ic_call(self.canister()?, "get_many", (keys,)).await }
    }

    fn get_all(
        &self,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        async move { ic_call(self.canister()?, "get_all", ()).await }
    }

    fn find(
        &self,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<Option<(K, V)>>> + Sync + Send {
        async move { ic_call(self.canister()?, "find", (filters,)).await }
    }

    fn filter(
        &self,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        async move { ic_call(self.canister()?, "filter", (filters,)).await }
    }

    fn insert(
        &self,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        async move { ic_call(self.canister()?, "insert", (key, value)).await }
    }

    fn update(
        &self,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        async move { ic_call(self.canister()?, "update", (key, value)).await }
    }

    fn remove(
        &self,
        key: K,
    ) -> impl std::future::Future<Output = CanisterResult<bool>> + Sync + Send {
        async move { ic_call(self.canister()?, "remove", (key,)).await }
    }

    fn remove_many(
        &self,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<()>> + Sync + Send {
        async move { ic_call(self.canister()?, "remove_many", (keys,)).await }
    }
}
