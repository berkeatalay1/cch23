pub mod endpoints;
pub mod models;

use actix_web::web::ServiceConfig;
use endpoints::{qminus1::{hello_world, error_example}, q1::xor_pow_endpoint, q4::{reindeer_strength_endpoint, reindeer_contest_endpoint}, q6::elf_string_endpoint, q7::{cookie_endpoint, bake_cookie_endpoint}};
use shuttle_actix_web::ShuttleActixWeb;



#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
        .service(error_example)
        .service(xor_pow_endpoint)
        .service(reindeer_strength_endpoint)
        .service(reindeer_contest_endpoint)
        .service(elf_string_endpoint)
        .service(cookie_endpoint)
        .service(bake_cookie_endpoint);
    };

    Ok(config.into())
}
