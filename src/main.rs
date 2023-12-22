pub mod endpoints;
pub mod models;

use std::collections::HashMap;

use actix_web::web::ServiceConfig;
use endpoints::{qminus1::{hello_world, error_example}, q1::xor_pow_endpoint, q4::{reindeer_strength_endpoint, reindeer_contest_endpoint}, q6::elf_string_endpoint, q7::{cookie_endpoint, bake_cookie_endpoint}, q8::{pokemon_weigth_endpoint, pokemon_drop_endpoint}, q11::{decoration_png_endpoint, count_super_red_endpoint}, q12::day12_routes};
use shuttle_actix_web::ShuttleActixWeb;
use tokio::{sync::RwLock, time::Instant};

#[derive(Default)]
struct Day12State {
    packets: RwLock<HashMap<String, Instant>>,
}

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
            .service(bake_cookie_endpoint)
            .service(pokemon_weigth_endpoint)
            .service(pokemon_drop_endpoint)
            .service(decoration_png_endpoint)
            .service(count_super_red_endpoint);
        day12_routes(cfg);

    };

    Ok(config.into())
}
