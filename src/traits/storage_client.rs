use candid::Principal;

use crate::{helpers::ic_call::ic_call, paged_response::PagedResponse, CanisterResult};

use super::{Filter, Sorter};

pub trait StorageClient<K, V, F, S>: Default + Send + Sync
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
        + Ord
        + Clone
        + Send
        + Sync,
    V: candid::CandidType + for<'a> candid::Deserialize<'a> + Sync + Send,
    F: Filter<K, V> + candid::CandidType + Clone + Send + Sync,
    S: Sorter<K, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    fn canister(&self) -> CanisterResult<Principal>;

    fn size(&self) -> impl std::future::Future<Output = CanisterResult<u64>> + Sync + Send {
        async move { ic_call(self.canister()?, "size", ()).await }
    }

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

    fn get_paginated(
        &self,
        limit: usize,
        page: usize,
        sort: S,
    ) -> impl std::future::Future<Output = CanisterResult<PagedResponse<(K, V)>>> + Sync + Send
    {
        async move {
            let args = (limit, page, sort);
            ic_call(self.canister()?, "get_paginated", args).await
        }
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

    fn filter_paginated(
        &self,
        limit: usize,
        page: usize,
        sort: S,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<PagedResponse<(K, V)>>> + Sync + Send
    {
        async move {
            let args = (limit, page, sort, filters);
            ic_call(self.canister()?, "filter_paginated", args).await
        }
    }

    fn update(
        &self,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        async move { ic_call(self.canister()?, "update", (key, value)).await }
    }

    fn update_many(
        &self,
        list: Vec<(K, V)>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        async move { ic_call(self.canister()?, "update_many", (list,)).await }
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

pub trait StorageClientInsertable<V, F, S>: StorageClient<u64, V, F, S>
where
    V: candid::CandidType + for<'a> candid::Deserialize<'a> + Sync + Send,
    F: Filter<u64, V> + candid::CandidType + Clone + Send + Sync,
    S: Sorter<u64, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    fn insert(
        &self,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(u64, V)>> + Sync + Send {
        async move { ic_call(self.canister()?, "insert", (value,)).await }
    }
}

pub trait StorageClientInsertableByKey<K, V, F, S>: StorageClient<K, V, F, S>
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
        + Ord
        + Clone
        + Send
        + Sync,
    V: candid::CandidType + for<'a> candid::Deserialize<'a> + Sync + Send,
    F: Filter<K, V> + candid::CandidType + Clone + Send + Sync,
    S: Sorter<K, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    fn insert(
        &self,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        async move { ic_call(self.canister()?, "insert", (key, value)).await }
    }
}
