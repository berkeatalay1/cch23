use std::io::Read;

use actix_multipart::form::MultipartForm;
use actix_web::{get, HttpRequest, Responder, web::Form, post, HttpResponse};
use image::GenericImageView;

use crate::models::q11::Image;

#[get("/11/assets/decoration.png")]
async fn decoration_png_endpoint(_req:HttpRequest) ->  impl Responder  {
    actix_files::NamedFile::open_async("assets/Shuttle.png").await
}

#[post("/11/red_pixels")]
async fn count_super_red_endpoint(MultipartForm(form): MultipartForm<Image>,) ->  impl Responder  {
    let img = image::load_from_memory(&form.image.data).unwrap();
    let count = img.pixels().filter(|(_,_,rgba)| rgba.0[0] as i32 > rgba.0[1] as i32 + rgba.0[2] as i32 ).count();
    HttpResponse::Ok().json(count)

}