mod config;
mod db;
mod errors;
mod routes;
mod user;
mod utils;

use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use firestore::FirestoreDb;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  log4rs::init_file("log4rs.yml", Default::default()).unwrap();

  let config = config::Config::from_env();
  info!("Serving on port: {}", config.port);

  HttpServer::new(move || {
    let logger = Logger::default();

    App::new()
      .wrap(logger)
      .data_factory(|| {
        let config = config::Config::from_env();
        FirestoreDb::new(config.firestore_project)
      })
      .app_data(web::Data::new(config::Config::from_env()))
      .service(routes::submit_birthday)
      .service(routes::get_user)
      .service(routes::healthz)
      .app_data(web::JsonConfig::default().error_handler(|error, _| {
        error::InternalError::from_response(error, HttpResponse::BadRequest().finish()).into()
      }))
  })
  .bind(("0.0.0.0", config.port))?
  .run()
  .await
}
