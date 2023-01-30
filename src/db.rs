use firestore::FirestoreDb;

use crate::{errors::AppError, user::User};

const COLLECTION_NAME: &str = "users";

pub async fn store_user(db: &FirestoreDb, user: &User) -> Result<(), AppError> {
  db.fluent()
    .update()
    .in_col(COLLECTION_NAME)
    .document_id(user.name())
    .object(user)
    .execute()
    .await?;

  Ok(())
}

pub async fn retrieve_user(db: &FirestoreDb, name: &str) -> Result<User, AppError> {
  db.fluent()
    .select()
    .by_id_in(COLLECTION_NAME)
    .obj()
    .one(name)
    .await?
    .ok_or(AppError::NotFound)
}
