use actix_web::{web::Json, HttpResponse, post};

use crate::models::req_strength::ReqStrength;

#[post("/4/strength")]
async fn reindeer_strength_endpoint(strength_json: Json<Vec<ReqStrength>>) -> HttpResponse {
    let reindeer_list = strength_json.into_inner();
    
    let total_strength = reindeer_list.iter().map(|reindeer| reindeer.strength).reduce(|acc,e| acc + e);

    HttpResponse::Ok().json(total_strength)
}
