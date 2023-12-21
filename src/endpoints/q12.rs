

use actix_web::{get, Responder, post, HttpResponse, web::Path};
use tokio::time::Instant;

use crate::Day12State;



#[post("/12/save/{payload}")]
async fn save_payload(data: actix_web::web::Data<Day12State>, path: Path<String>) ->  HttpResponse {
    let mut packets = data.packets.write().await;
    packets.insert(path.into_inner(), Instant::now());
    HttpResponse::Ok().finish()
}

#[get("/12/load/{payload}")]
async fn get_payload(data: actix_web::web::Data<Day12State>, path: Path<String>,) ->  HttpResponse {
    let packets = data.packets.read().await;
    let instant = packets.get(&path.into_inner()).unwrap();
    HttpResponse::Ok().body(instant.elapsed().as_secs().to_string())
}