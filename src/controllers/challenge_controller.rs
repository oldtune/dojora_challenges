use crate::diesel::RunQueryDsl;
use crate::model::Challenge;
use crate::schema::challenge;
use crate::{
    response_models::base_response_model::BaseResponseModel,
    view_models::{
        create_challenge_view_model::CreateChallengeViewModel,
        validator::{ModelValidator, Validity},
    },
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
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
    let connection = establish_connection();
    let new_challenge = Challenge {
        title: challenge_to_create.title.clone(),
        description: challenge_to_create.description.clone(),
        id: uuid::Uuid::new_v4(),
    };

    diesel::insert_into(challenge::table)
        .values(&new_challenge)
        .execute(&connection)
        .expect("something went wrong");

    HttpResponse::Ok()
        .body(serde_json::to_string(&BaseResponseModel::<'_, ()>::empty_success()).unwrap())
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect("Failed to connect to db")
}
