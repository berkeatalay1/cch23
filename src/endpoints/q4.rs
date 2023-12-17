use actix_web::{web::Json, HttpResponse, post};

use crate::models::q4::{ReqStrength, ReqContest, ResContest};

#[post("/4/strength")]
async fn reindeer_strength_endpoint(strength_json: Json<Vec<ReqStrength>>) -> HttpResponse {
    let reindeer_list = strength_json.into_inner();
    
    let total_strength = reindeer_list.iter().map(|reindeer| reindeer.strength).reduce(|acc,e| acc + e);

    HttpResponse::Ok().json(total_strength)
}

#[post("/4/contest")]
async fn reindeer_contest_endpoint(strength_json: Json<Vec<ReqContest>>) -> HttpResponse {
    let reindeer_list = strength_json.into_inner();
    let mut fastest = ReqContest::default();
    let mut tallest = ReqContest::default();
    let mut magician = ReqContest::default();
    let mut consumer = ReqContest::default();
    for reindeer in reindeer_list{
        if reindeer.speed > fastest.speed{
            fastest = reindeer.clone()
        }
        if reindeer.height > tallest.height{
            tallest = reindeer.clone()
        }
        if reindeer.snow_magic_power > magician.snow_magic_power{
            magician =reindeer.clone()
        }
        if reindeer.candies_eaten > consumer.candies_eaten{
            consumer = reindeer.clone()
        }
    }
    let response = ResContest{ 
        fastest: format!("Speeding past the finish line with a strength of {} is {}",fastest.strength,fastest.name), 
        tallest: format!("{} is standing tall with his {} cm wide antlers",tallest.name,tallest.antler_width), 
        magician: format!("{} could blast you away with a snow magic power of {}",magician.name,magician.snow_magic_power), 
        consumer: format!("{} ate lots of candies, but also some {}",consumer.name,consumer.favorite_food) 
    };

    HttpResponse::Ok().json(response)
}
