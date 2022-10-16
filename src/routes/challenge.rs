use actix_web::HttpResponse;

pub async fn get_all_challenges() -> HttpResponse {
    HttpResponse::Ok().finish()
}
