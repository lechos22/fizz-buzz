use std::error::Error;

use actix_web::{get, web, HttpResponse};

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
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
            .service(nth_entry_endpoint)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;
    Ok(())
}
