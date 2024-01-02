use std::error::Error;

use actix_web::{get, web, HttpResponse};
use log::info;

const ADDRESS: &str = "0.0.0.0:3000";

fn index_html() -> &'static str {
    include_str!("./index.html")
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(index_html())
}

fn nth_entry(n: u64) -> String {
    match n {
        n if n % 15 == 0 => "Fizz Buzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        n => n.to_string(),
    }
}

#[get("/nth/{n}")]
async fn nth_entry_endpoint(path: web::Path<u64>) -> String {
    serde_json::to_string(&nth_entry(path.into_inner())).expect("Failed to serialize response")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use actix_web::{middleware::Logger, App, HttpServer};
    use env_logger::Env;

    let env = Env::new().filter_or("RUST_LOG", "info");
    env_logger::try_init_from_env(env)?;
    info!("Server starting at http://{}", ADDRESS);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(nth_entry_endpoint)
    })
    .bind(ADDRESS)?
    .run()
    .await?;
    Ok(())
}
