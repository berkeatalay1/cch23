use actix_web::{HttpRequest, HttpResponse,  get};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{Value, json};

use crate::models::q7::ResBakeCookies;


#[get("/7/decode")]
async fn cookie_endpoint(req:HttpRequest) -> HttpResponse {

    let opt_header = req.cookie("recipe").unwrap();
    let recipe_header = opt_header.value();
    let recipe_binary = &general_purpose::STANDARD.decode(recipe_header).unwrap();
    let recipe_json = std::str::from_utf8(recipe_binary.as_ref()).unwrap();

    let recipe:Value = serde_json::from_str(recipe_json).unwrap();
    HttpResponse::Ok().json(recipe)
}

#[get("/7/bake")]
async fn bake_cookie_endpoint(req:HttpRequest) -> HttpResponse {

    let opt_header = req.cookie("recipe").unwrap();
    let recipe_header = opt_header.value();
    let recipe_binary = &general_purpose::STANDARD.decode(recipe_header).unwrap();
    let recipe_json = std::str::from_utf8(recipe_binary.as_ref()).unwrap();

    let bake_json:Value = serde_json::from_str(recipe_json).unwrap();

    let recipe = bake_json["recipe"].to_owned().as_object().unwrap().clone();
    let pantry = bake_json["pantry"].to_owned().as_object().unwrap().clone();
    let mut total_cookies_possible = -1;
    for (key,value) in pantry.clone(){
        let value_int:i64 = serde_json::from_value(value).unwrap();
        let recipe_int:i64 = serde_json::from_value(recipe.get(&key).unwrap_or(&json!(0)).to_owned()).unwrap();
        let total_possible = if recipe_int > 0 {value_int / recipe_int} else{9999999999};
        if total_cookies_possible < 0 {
            total_cookies_possible = total_possible
        }
        if total_cookies_possible > total_possible {
            total_cookies_possible = total_possible
        }
    }

    let mut response_value = serde_json::json!({});
    for (key,value) in pantry.clone(){
        let current_value:i64 = serde_json::from_value(value).unwrap();
        let required_value:i64 = (serde_json::from_value::<i64>(recipe.get(&key).unwrap_or(&json!(0)).clone()).unwrap()) * total_cookies_possible;
        let remaning = current_value - required_value;
        response_value[key] = serde_json::to_value(remaning).unwrap();

    };

    let response = ResBakeCookies{
        cookies: total_cookies_possible,
        pantry: response_value
    };
    println!("---------------------- response -------------------------");
    println!("{:?}",serde_json::json!(response));
    println!("---------------------- response -------------------------");

    HttpResponse::Ok().json(response)
}