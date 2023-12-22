

use actix_web::{get, post, HttpResponse, web::{Path, ServiceConfig}};
use tokio::time::Instant;
use ulid::Ulid;
use uuid::Uuid;
use chrono::{DateTime, Datelike, Utc, Weekday};

use crate::{Day12State, models::q12::ResUlit};



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

#[post("/12/ulids")]
async fn ulit_to_uuid(json:actix_web::web::Json<Vec<String> >) ->  HttpResponse {
    let data = json.into_inner();
    let data_ulid = data.iter().map(|item| Ulid::from_string(item).unwrap()).collect::<Vec<Ulid>>();
    let mut data_uuid:Vec<Uuid> = data_ulid.into_iter().map(|item| item.into()).collect();
    data_uuid.reverse();
    HttpResponse::Ok().json(data_uuid)
}

#[post("/12/ulids/{weekdays}")]
async fn ulit_weekdays(json:actix_web::web::Json<Vec<String>>, path: Path<u32>) ->  HttpResponse {
    let data = json.into_inner();
    let weekday = path.into_inner();
    let data_ulid = data.iter().map(|item| Ulid::from_string(item).unwrap()).collect::<Vec<Ulid>>();
    let mut christmas_count = 0;
    let mut weekday_count = 0;
    let mut future_cout = 0;
    let mut lsb_count = 0;

    for ulid in data_ulid.iter() {
        let date:DateTime<Utc> = ulid.datetime().into();
        if date.day() == 24 && date.month() == 12{
            christmas_count+=1;
        }
        if date.weekday().number_from_monday() == weekday+1 {
            weekday_count+=1;
        }
        if date > Utc::now(){
            future_cout+=1;
        }
        if ulid.0 & 1 == 1{
            lsb_count+=1;
        }
    }
    let response = ResUlit{
        christmas: christmas_count,
        weekday: weekday_count,
        lsb: lsb_count,
        future: future_cout
        
    };
   
    HttpResponse::Ok().json(response)
}

pub fn day12_routes(cfg: &mut ServiceConfig) {
    cfg.app_data(actix_web::web::Data::new(Day12State::default()));
    cfg.service(save_payload);
    cfg.service(get_payload);
    cfg.service(ulit_to_uuid);
    cfg.service(ulit_weekdays);
}

