use chrono::{NaiveDate, Utc};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct User {
  name: String,
  #[serde(rename = "dateOfBirth")]
  birthday: NaiveDate,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PutUserBody {
  #[serde(rename = "dateOfBirth")]
  pub birthday: NaiveDate,
}

fn validate_name(name: &str) -> Result<(), AppError> {
  lazy_static! {
    static ref NAME_VALIDATOR: Regex = RegexBuilder::new(r"^[a-z]+$")
      .case_insensitive(true)
      .build()
      .expect("regex to compile");
  }

  // Return an error if the name isn't just letters
  if !NAME_VALIDATOR.is_match(name) {
    return Err(AppError::BadRequest(
      "Invalid name, only letters allowed.".into(),
    ));
  }

  Ok(())
}

fn validate_birthday(birthday: &NaiveDate) -> Result<(), AppError> {
  if Utc::now().date_naive() < *birthday {
    return Err(AppError::BadRequest(
      "Provided birthday is in the future!".into(),
    ));
  }

  Ok(())
}

impl User {
  pub fn new(name: String, birthday: NaiveDate) -> Result<User, AppError> {
    // Perform validation
    validate_name(&name)?;
    validate_birthday(&birthday)?;

    Ok(User { name, birthday })
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn birthday(&self) -> &NaiveDate {
    &self.birthday
  }
}

#[cfg(test)]
mod tests {
  use chrono::NaiveDate;

  use crate::errors::AppError;

  #[test]
  fn new_user_valid() {
    let name = "dylan";
    let birthday = NaiveDate::from_ymd_opt(1990, 1, 12).unwrap();
    let user = super::User::new(name.into(), birthday);

    assert_eq!(user.is_ok(), true);
    assert_eq!(
      user.unwrap(),
      super::User {
        name: name.into(),
        birthday: birthday
      }
    );
  }

  #[test]
  fn new_user_invalid() {
    let name = "dy1an";
    let birthday = NaiveDate::from_ymd_opt(1990, 1, 12).unwrap();
    let user = super::User::new(name.into(), birthday);

    assert_eq!(user.is_err(), true);
    assert_eq!(
      user.unwrap_err(),
      AppError::BadRequest("Invalid name, only letters allowed.".into())
    );
  }

  #[test]
  fn validate_name_valid() {
    let name = "validname".into();
    let result = super::validate_name(name);

    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn validate_name_invalid() {
    let name = "INva1id_n4me".into();
    let result = super::validate_name(name);

    assert_eq!(result.is_err(), true);
    assert_eq!(
      result.unwrap_err(),
      AppError::BadRequest("Invalid name, only letters allowed.".into())
    );
  }
}
