use dojora::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::Config::default();
    run()?.await
}
