use actix_web::{get, HttpResponse};

#[get("/")]
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/-1/error")]
async fn error_example() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}