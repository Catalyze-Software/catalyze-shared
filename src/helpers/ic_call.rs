use candid::{utils::ArgumentEncoder, Principal};
use serde::de::DeserializeOwned;

use crate::{api_error::ApiError, CanisterCallResult, CanisterResult};

pub async fn ic_call<A: ArgumentEncoder, R: candid::CandidType + DeserializeOwned>(
    canister: Principal,
    method: &str,
    args: A,
) -> CanisterResult<R> {
    let fut = ic_cdk::call::<A, (CanisterCallResult<R>,)>(canister, method, args).await;

    let method: String = method.to_string();

    let (result,) = fut.map_err(|e| {
        ApiError::unexpected()
            .add_message("Failed to call canister")
            .add_info(format!("Canister: {canister} error: {:?}", e).as_str())
            .add_method_name(method)
    })?;

    result.into()
}
