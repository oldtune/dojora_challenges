use serde::Serialize;

use crate::view_models::validator::ErrorMessages;

#[derive(Serialize)]
pub struct BaseResponseModel<'a, T>
where
    T: Serialize,
{
    #[serde(rename(serialize = "validationResult"))]
    pub validation_result: Option<ErrorMessages<'a>>,
    #[serde(rename(serialize = "data"))]
    pub data: Option<T>,
    #[serde(rename(serialize = "status"))]
    pub status: ResponseStatus,
    pub errors: Option<ErrorMessages<'a>>,
}

#[derive(Serialize)]
pub enum ResponseStatus {
    Ok,
    Failed,
}

impl<'a, T> BaseResponseModel<'a, T>
where
    T: Serialize,
{
    pub fn from_err_message(error: ErrorMessages<'a>) -> Self {
        Self {
            data: None,
            validation_result: None,
            status: ResponseStatus::Failed,
            errors: Some(error),
        }
    }

    pub fn empty_success() -> Self {
        Self {
            validation_result: None,
            data: None,
            status: ResponseStatus::Ok,
            errors: None,
        }
    }

    pub fn empty_failed() -> Self {
        Self {
            validation_result: None,
            data: None,
            status: ResponseStatus::Failed,
            errors: None,
        }
    }

    pub fn validation_error(error: ErrorMessages<'a>) -> Self {
        Self {
            validation_result: Some(error),
            data: None,
            status: ResponseStatus::Failed,
            errors: None,
        }
    }

    pub fn success(data: T) -> Self {
        Self {
            validation_result: None,
            data: Some(data),
            status: ResponseStatus::Ok,
            errors: None,
        }
    }
}
