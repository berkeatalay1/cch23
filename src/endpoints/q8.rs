use actix_web::{get, HttpRequest, HttpResponse, web::Path};

use crate::models::q8::ResPokedexApi;

#[get("/8/weight/{pokemon_id}")]
async fn pokemon_weigth_endpoint(_req:HttpRequest,path: Path<i32>) -> HttpResponse {

    let pokemon_id = path.into_inner();
    let pokemon_info_str = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}",pokemon_id)).await.unwrap().text().await.unwrap();
    let pokemon_info:ResPokedexApi = serde_json::from_str(&pokemon_info_str).unwrap();
    HttpResponse::Ok().json((pokemon_info.weight as f64)/10.0)
}

#[get("/8/drop/{pokemon_id}")]
async fn pokemon_drop_endpoint(_req:HttpRequest,path: Path<i32>) -> HttpResponse {
    const G_POWER:f64 = 9.825;
    let pokemon_id = path.into_inner();
    let pokemon_info_str = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}",pokemon_id)).await.unwrap().text().await.unwrap();
    let pokemon_info:ResPokedexApi = serde_json::from_str(&pokemon_info_str).unwrap();
    let weight = pokemon_info.weight;
    let calculation_param = 2.0 * G_POWER * 10.0;
    let momentum = calculation_param.sqrt() * ((weight as f64) / 10.0) ;
    HttpResponse::Ok().json(momentum)
}