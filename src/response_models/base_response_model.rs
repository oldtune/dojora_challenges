use serde::Serialize;

use crate::view_models::validator::ErrorMessages;

#[derive(Serialize)]
pub struct BaseResponseModel<'a, T>
where
    T: Serialize,
{
    #[serde(rename(serialize="validationResult"))]
    pub validation_result: Option<ErrorMessages<'a>>,
    #[serde(rename(serialize = "data"))]
    pub data: Option<ResponseData<T>>,
    #[serde(rename(serialize = "status"))]
    pub status: ResponseStatus,
}

#[derive(Serialize)]
pub struct ResponseData<T>
where
    T: Serialize,
{
    data: T,
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
            validation_result: Some(error),
            status: ResponseStatus::Ok,
        }
    }

    pub fn empty_success() -> Self {
        Self {
            validation_result: None,
            data: None,
            status: ResponseStatus::Ok,
        }
    }

    pub fn empty_failed() -> Self {
        Self {
            validation_result: None,
            data: None,
            status: ResponseStatus::Failed,
        }
    }
}
