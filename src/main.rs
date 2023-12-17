pub mod endpoints;

use actix_web::web::ServiceConfig;
use endpoints::qminus1::{hello_world, error_example};
use shuttle_actix_web::ShuttleActixWeb;



#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
        .service(error_example);
    };

    Ok(config.into())
}
