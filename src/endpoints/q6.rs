use actix_web::{post, HttpResponse, web::{Json, self}};

use crate::models::q6::ResElfString;


#[post("/6")]
async fn elf_string_endpoint(body: web::Bytes) -> HttpResponse {
    let elf_string = std::str::from_utf8(&body).unwrap();

    let count = elf_string.matches("elf").count();

    let response = ResElfString{elf:count};
    HttpResponse::Ok().json(response)
}