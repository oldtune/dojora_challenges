use crate::diesel::RunQueryDsl;
use crate::helpers::db_pool::DbPool;
use crate::helpers::query_helper::Paginate;
use crate::model::Challenge;
use crate::schema::challenge;
use crate::schema::challenge::dsl::*;
use crate::view_models::challenge_view_model::ChallengeViewModel;
use crate::view_models::paging_request_model::PagingModel;
use crate::{
    response_models::base_response_model::BaseResponseModel,
    view_models::{
        create_challenge_view_model::CreateChallengeViewModel,
        validator::{ModelValidator, Validity},
    },
};

use actix_web::web::Data;
use actix_web::{get, Result};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::query_builder::AsQuery;
use diesel::QueryDsl;
use std::str::FromStr;

#[post("api/challenges")]
pub async fn create_challenge(
    db_pool: Data<DbPool>,
    challenge_to_create: web::Json<CreateChallengeViewModel>,
) -> impl Responder {
    if let Validity::Invalid(err) = challenge_to_create.validate() {
        let response_message = BaseResponseModel::<'_, ()>::from_err_message(err);
        let json = serde_json::to_string(&response_message).unwrap();
        return HttpResponse::BadRequest().body(json);
    }

    //insert into db
    let mut connection = db_pool.get().unwrap();
    let new_challenge = Challenge {
        title: challenge_to_create.title.clone(),
        description: Some(challenge_to_create.description.clone()),
        id: uuid::Uuid::new_v4(),
    };

    let insert_result = diesel::insert_into(challenge::table)
        .values(&new_challenge)
        .execute(&mut connection);

    match insert_result {
        Ok(_) => HttpResponse::Ok()
            .body(serde_json::to_string(&BaseResponseModel::<'_, ()>::empty_success()).unwrap()),
        Err(_) => {
            let response =
                BaseResponseModel::<'_, ()>::from_err_message(vec!["Failed to insert to db"]);
            HttpResponse::InternalServerError().body(serde_json::to_string(&response).unwrap())
        }
    }
}

#[get("api/challenges")]
pub async fn get_challenges(
    db_pool: Data<DbPool>,
    paging_model: web::Json<PagingModel>,
) -> Result<impl Responder> {
    let mut connection = db_pool.get().unwrap();
    let challenges = challenge
        .as_query()
        .paginate(paging_model.page_index, paging_model.page_size)
        .load_paging(&mut connection)
        .expect("Failed to load challenges");

    let view_model: Vec<ChallengeViewModel> = challenges
        .into_iter()
        .map(ChallengeViewModel::new)
        .collect();

    Ok(web::Json(BaseResponseModel::success(view_model)))
}

#[get("api/challenges/{challenge_id}")]
pub async fn get_challenge_detail(
    db_pool: Data<DbPool>,
    challenge_id: web::Path<String>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();
    let uuid_parsed = uuid::Uuid::from_str(&challenge_id);
    if uuid_parsed.is_err() {
        return web::Json(BaseResponseModel::from_err_message(vec![
            "Invalid id format",
        ]));
    }

    let individual_challenge = challenge.find(uuid_parsed.unwrap()).first(&mut connection);

    match individual_challenge {
        Err(_) => web::Json(BaseResponseModel::from_err_message(vec![
            "Failed to find challenge",
        ])),
        Ok(found_challenge) => {
            let challenge_response = ChallengeViewModel::new(found_challenge);
            web::Json(BaseResponseModel::success(challenge_response))
        }
    }
}
