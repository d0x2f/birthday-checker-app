use crate::config::Config;
use crate::errors::AppError;
use crate::models::*;
use crate::utils;
use actix_web::Responder;
use actix_web::{get, put, web, HttpResponse};
use chrono::Utc;
use firestore::FirestoreDb;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serde_json::json;

const COLLECTION_NAME: &str = "users";

#[put("/hello/{name}")]
async fn submit_birthday(
  config: web::Data<Config>,
  params: web::Path<String>,
  body: web::Json<PutUserBody>,
) -> Result<HttpResponse, AppError> {
  lazy_static! {
    static ref NAME_VALIDATOR: Regex = RegexBuilder::new(r"^[a-z]+$")
      .case_insensitive(true)
      .build()
      .expect("regex to compile");
  }

  let db = FirestoreDb::new(config.firestore_project.clone()).await?;

  let name = params.into_inner();
  let birthday = body.into_inner().birthday;

  // Return an error if the name isn't just letters
  if !NAME_VALIDATOR.is_match(&name) {
    return Err(AppError::BadRequest(
      "Invalid name, only letters allowed.".into(),
    ));
  }

  // Return an error if the birthday is in the future
  if Utc::now().date_naive() < birthday {
    return Err(AppError::BadRequest(
      "Provided birthday is in the future!".into(),
    ));
  }

  let user = User { name, birthday };

  db.fluent()
    .update()
    .in_col(COLLECTION_NAME)
    .document_id(&user.name)
    .object(&user)
    .execute()
    .await?;

  Ok(HttpResponse::NoContent().finish())
}

#[get("/hello/{name}")]
async fn get_user(
  config: web::Data<Config>,
  params: web::Path<String>,
) -> Result<HttpResponse, AppError> {
  let db = FirestoreDb::new(config.firestore_project.clone()).await?;

  let name = params.into_inner();

  let user: User = db
    .fluent()
    .select()
    .by_id_in(COLLECTION_NAME)
    .obj()
    .one(&name)
    .await?
    .ok_or(AppError::NotFound)?;

  let message = match utils::days_until_next_birthday(Utc::now().date_naive(), user.birthday)? {
    0 => format!("Hello, {}! Happy birthday!", user.name),
    n => format!("Hello, {}! Your birthday is in {} day(s)", user.name, n),
  };

  Ok(HttpResponse::Ok().json(json!({ "message": message })))
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
  HttpResponse::Ok().finish()
}
