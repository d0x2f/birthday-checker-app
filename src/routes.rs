use crate::db::{retrieve_user, store_user};
use crate::errors::AppError;
use crate::user::{PutUserBody, User};
use crate::utils;
use actix_web::Responder;
use actix_web::{get, put, web, HttpResponse};
use chrono::Utc;
use firestore::FirestoreDb;
use serde_json::json;

#[put("/hello/{name}")]
async fn submit_birthday(
  db: web::Data<FirestoreDb>,
  params: web::Path<String>,
  body: web::Json<PutUserBody>,
) -> Result<HttpResponse, AppError> {
  let name = params.into_inner();
  let birthday = body.into_inner().birthday;

  let user = User::new(name, birthday)?;
  store_user(&db, &user).await?;

  Ok(HttpResponse::NoContent().finish())
}

#[get("/hello/{name}")]
async fn get_user(
  db: web::Data<FirestoreDb>,
  params: web::Path<String>,
) -> Result<HttpResponse, AppError> {
  let name = params.into_inner();
  let user: User = retrieve_user(&db, &name).await?;
  let message = utils::get_birthday_message(Utc::now().date_naive(), &user)?;

  Ok(HttpResponse::Ok().json(json!({ "message": message })))
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
  HttpResponse::Ok().finish()
}
