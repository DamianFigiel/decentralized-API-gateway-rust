use actix_web::{get, post, web, App, HttpServer, HttpResponse, HttpRequest, Responder, Error};
use serde::{Serialize, Deserialize};
mod jwt;
mod ws_handler;

#[derive(Serialize, Deserialize)]
struct AuthRequest {
    user_id: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Decentralized API Gateway!")
}

#[post("/auth")]
async fn auth(data: web::Json<AuthRequest>) -> impl Responder {
    let token = jwt::create_jwt(&data.user_id);
    HttpResponse::Ok().json(AuthResponse { token })
}


#[get("/secure")]
async fn secure(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            let token = auth_str.trim_start_matches("Bearer ");
            match jwt::validate_jwt(token) {
                Ok(_) => Ok(HttpResponse::Ok().body("Secure endpoint access granted.")),
                Err(_) => Ok(HttpResponse::Unauthorized().body("Invalid JWT token."))
            }
        } else {
            Ok(HttpResponse::BadRequest().body("Invalid Authorization header."))
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("Authorization header is missing."))
    }
}

#[get("/ws")]
async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws_handler::ws_index(req, stream).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Decentralized API Gateway is listening on port 9090");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(auth)
            .service(secure)
            .service(websocket_route)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}