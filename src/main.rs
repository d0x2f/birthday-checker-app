mod config;
mod errors;
mod models;
mod routes;
mod utils;

use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  log4rs::init_file("log4rs.yml", Default::default()).unwrap();
  let config = config::Config::from_env();

  info!("Serving on port: {}", config.port);

  HttpServer::new(|| {
    let logger = Logger::default();
    App::new()
      .wrap(logger)
      .app_data(web::Data::new(config::Config::from_env()))
      .service(routes::submit_birthday)
      .service(routes::get_user)
      .app_data(web::JsonConfig::default().error_handler(|error, _| {
        error::InternalError::from_response(error, HttpResponse::BadRequest().finish()).into()
      }))
  })
  .bind(("0.0.0.0", config.port))?
  .run()
  .await
}
