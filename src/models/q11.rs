use actix_multipart::form::{MultipartForm, bytes::Bytes};
#[derive(Debug,MultipartForm)]
pub struct Image{
    pub image:Bytes,
}