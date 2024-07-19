use candid::{utils::ArgumentEncoder, Principal};

use crate::{api_error::ApiError, CanisterCallResult, CanisterResult};

pub fn ic_call<A: ArgumentEncoder, R: candid::CandidType + for<'a> candid::Deserialize<'a>>(
    canister: Principal,
    method: &str,
    args: A,
) -> impl std::future::Future<Output = CanisterResult<R>> + Sync + Send {
    let fut = ic_cdk::call::<A, (CanisterCallResult<R>,)>(canister, method, args);

    let method: String = method.to_string();

    async move {
        let (res,) = fut.await.map_err(|e| {
            ApiError::unexpected()
                .add_message("Failed to call canister")
                .add_info(format!("Canister: {canister} error: {:?}", e).as_str())
                .add_method_name(method)
        })?;

        res.into()
    }
}
