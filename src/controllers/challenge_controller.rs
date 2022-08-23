use crate::{
    response_models::base_response_model::BaseResponseModel,
    view_models::{
        create_challenge_view_model::CreateChallengeViewModel,
        validator::{ModelValidator, Validity},
    },
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("api/challenges")]
pub async fn create_challenge(challenge: web::Json<CreateChallengeViewModel>) -> impl Responder {
    if let Validity::Invalid(err) = challenge.validate() {
        let response_message = BaseResponseModel::<'_, ()>::from_err_message(err);
        let json = serde_json::to_string(&response_message).unwrap();
        return HttpResponse::BadRequest().body(json);
    }

    HttpResponse::Ok()
        .body(serde_json::to_string(&BaseResponseModel::<'_, ()>::empty_success()).unwrap())
}
