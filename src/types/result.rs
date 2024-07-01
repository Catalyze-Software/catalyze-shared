use crate::api_error::ApiError;

pub type CanisterResult<T> = Result<T, ApiError>;
