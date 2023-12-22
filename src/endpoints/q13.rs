use actix_web::{web::{ServiceConfig, Json}, HttpResponse,  get, post};
use serde_json::json;

use crate::{AppState, models::q13::Order};

#[get("/13/sql")]
async fn sqlx_hello(data: actix_web::web::Data<AppState>) ->  HttpResponse {
    let response = sqlx::query!("SELECT 20231213 number").fetch_one(&data.pool).await.unwrap();
    HttpResponse::Ok().json(response.number)
}

#[post("/13/reset")]
async fn db_reset(data: actix_web::web::Data<AppState>) ->  HttpResponse {
    sqlx::query!("DROP TABLE IF EXISTS orders").execute(&data.pool).await.unwrap();
    sqlx::query!("CREATE TABLE orders (
        id INT PRIMARY KEY,
        region_id INT,
        gift_name VARCHAR(50),
        quantity INT
      )").execute(&data.pool).await.unwrap();

    HttpResponse::Ok().finish()
}

#[post("/13/orders")]
async fn order(data: actix_web::web::Data<AppState>,Json(orders): Json<Vec<Order>>) ->  HttpResponse {
    for order in orders{
        sqlx::query!(
            "INSERT INTO orders(id, region_id, gift_name, quantity) values($1, $2, $3, $4)",
            order.id,
            order.region_id,
            order.gift_name,
            order.quantity)
            .execute(&data.pool).await.unwrap();
    }

    HttpResponse::Ok().finish()
}

#[get("/13/orders/total")]
async fn total_orders(data: actix_web::web::Data<AppState>) ->  HttpResponse {
    let quantity = sqlx::query!("Select SUM(quantity) as qt from orders").fetch_one(&data.pool).await.unwrap().qt;

    HttpResponse::Ok().json(json!({"total":quantity}))
}

#[get("/13/orders/popular")]
async fn popular_ordre(data: actix_web::web::Data<AppState>) ->  HttpResponse {
    let gift_name = match sqlx::query!("Select gift_name, SUM(quantity) as qt from orders group by gift_name order by qt desc limit 1").fetch_one(&data.pool).await{
            Ok(ok) => ok.gift_name,
            Err(_) => None
        };
    HttpResponse::Ok().json(json!({"popular":gift_name}))
}

pub fn day13_routes(cfg: &mut ServiceConfig) {
    cfg.service(sqlx_hello);
    cfg.service(db_reset);
    cfg.service(order);
    cfg.service(total_orders);
    cfg.service(popular_ordre);

}
