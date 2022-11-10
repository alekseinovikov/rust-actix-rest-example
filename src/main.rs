extern crate env_logger;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(api::metrics::metrics_middleware())
            .service(api::controller::hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
