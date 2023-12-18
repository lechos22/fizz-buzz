use std::error::Error;

use actix_web::get;

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(index))
        .bind("0.0.0.0:3000")?
        .run()
        .await?;
    Ok(())
}
