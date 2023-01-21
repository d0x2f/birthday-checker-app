use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
  pub name: String,
  #[serde(rename = "dateOfBirth")]
  pub birthday: NaiveDate,
}

#[derive(Deserialize)]
pub struct PutUserBody {
  #[serde(rename = "dateOfBirth")]
  pub birthday: NaiveDate,
}
