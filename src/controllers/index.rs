use actix_web::{get, web, HttpResponse, Responder, Result};

use crate::models::challenge_view_model::ChallengeViewModel;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/challenges")]
pub async fn get_challenges() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[get("challenges/{challengeId}")]
pub async fn get_challenges_detail() -> Result<impl Responder> {
    let challenge = ChallengeViewModel {
        description: "Description".to_string(),
        title: "Title".to_string(),
    };

    Ok(web::Json(challenge))
}
