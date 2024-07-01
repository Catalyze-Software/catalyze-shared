use candid::Principal;
use ic_cdk::caller;

use crate::api_error::ApiError;

pub fn is_not_anonymous() -> Result<(), String> {
    match caller() == Principal::anonymous() {
        true => Err(ApiError::unauthorized()
            .add_message("Anonymous principal")
            .to_string()),
        false => Ok(()),
    }
}
