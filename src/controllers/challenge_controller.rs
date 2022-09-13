use crate::diesel::RunQueryDsl;
use crate::helpers::query_helper::{Paginate, Paginated};
use crate::model::Challenge;
use crate::schema::challenge;
use crate::schema::challenge::dsl::*;
use crate::view_models::challenge_view_model::ChallengeViewModel;
use crate::view_models::paging_model::PagingModel;
use crate::{
    response_models::base_response_model::BaseResponseModel,
    view_models::{
        create_challenge_view_model::CreateChallengeViewModel,
        validator::{ModelValidator, Validity},
    },
};

use actix_web::{get, Result};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::query_builder::AsQuery;
use diesel::QueryDsl;
use diesel::{pg::PgConnection, Connection};
use std::env;

#[post("api/challenges")]
pub async fn create_challenge(
    challenge_to_create: web::Json<CreateChallengeViewModel>,
) -> impl Responder {
    if let Validity::Invalid(err) = challenge_to_create.validate() {
        let response_message = BaseResponseModel::<'_, ()>::from_err_message(err);
        let json = serde_json::to_string(&response_message).unwrap();
        return HttpResponse::BadRequest().body(json);
    }

    //insert into db
    let mut connection = establish_connection();
    let new_challenge = Challenge {
        title: challenge_to_create.title.clone(),
        description: Some(challenge_to_create.description.clone()),
        id: uuid::Uuid::new_v4(),
    };

    diesel::insert_into(challenge::table)
        .values(&new_challenge)
        .execute(&mut connection)
        .expect("something went wrong");

    HttpResponse::Ok()
        .body(serde_json::to_string(&BaseResponseModel::<'_, ()>::empty_success()).unwrap())
}

#[get("api/challenges")]
pub async fn get_challenges(paging_model: web::Json<PagingModel>) -> Result<impl Responder> {
    let mut connection = establish_connection();
    let challenges = challenge
        .as_query()
        .paginate(paging_model.page_index, paging_model.page_size)
        .load_paging(&mut connection)
        .expect("Failed to load challenges");

    let view_model: Vec<ChallengeViewModel> = challenges
        .into_iter()
        .map(move |x| ChallengeViewModel::new(x))
        .collect();

    Ok(web::Json(BaseResponseModel::success(view_model)))
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Failed to connect to db")
}
