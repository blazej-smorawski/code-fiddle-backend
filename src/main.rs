use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserCode {
    code: Vec<String>,
}

/// This handler uses json extractor with limit
async fn send_code(item: web::Json<UserCode>, req: HttpRequest) -> HttpResponse {
    println!("model: {item:?}");

    HttpResponse::Ok().json(item.0) // <- send json response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/send_code").route(web::post().to(send_code)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}