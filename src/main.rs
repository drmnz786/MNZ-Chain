use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("MNZ-Chain Service Running")
}

async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/api/health", web::get().to(health))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
