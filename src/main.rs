use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Decentralized API Gateway!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Decentralized API Gateway is listening on port 9090");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}