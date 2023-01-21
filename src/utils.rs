use crate::errors::AppError;
use chrono::{Datelike, NaiveDate};
use std::convert::TryInto;

// Accepting today's date enables testing without mocks
pub fn days_until_next_birthday(today: NaiveDate, birthday: NaiveDate) -> Result<u32, AppError> {
  let today_ordinal = today.ordinal();
  let birthday_ordinal = birthday.ordinal();

  // Birthday is today
  if today.ordinal() == birthday_ordinal {
    return Ok(0);
  }

  // Birthday yet to come this year
  if birthday_ordinal > today_ordinal {
    return Ok(birthday_ordinal - today_ordinal);
  }

  // Birthday already passed this year
  let next_birthday = birthday
    .with_year(today.year() + 1)
    .ok_or_else(|| AppError::Other("Date arithmetic overflow".into()))?;

  next_birthday
    .signed_duration_since(today)
    .num_days()
    .try_into()
    .map_err(|_| AppError::Other("Date arithmetic overflow".into()))
}

#[cfg(test)]
mod tests {
  use chrono::NaiveDate;

  #[test]
  fn birthday_is_today_exactly() {
    let today = NaiveDate::from_ymd_opt(1990, 1, 12).unwrap();
    let birthday = NaiveDate::from_ymd_opt(1990, 1, 12).unwrap();
    let result = super::days_until_next_birthday(today, birthday).unwrap();

    assert_eq!(result, 0);
  }

  #[test]
  fn birthday_is_today_different_year() {
    let today = NaiveDate::from_ymd_opt(2023, 1, 12).unwrap();
    let birthday = NaiveDate::from_ymd_opt(1990, 1, 12).unwrap();
    let result = super::days_until_next_birthday(today, birthday).unwrap();

    assert_eq!(result, 0);
  }

  #[test]
  fn birthday_is_yet_to_come_this_year() {
    let today = NaiveDate::from_ymd_opt(2023, 1, 12).unwrap();
    let birthday = NaiveDate::from_ymd_opt(1990, 6, 13).unwrap();
    let result = super::days_until_next_birthday(today, birthday).unwrap();

    assert_eq!(result, 152);
  }

  #[test]
  fn birthday_has_already_passed_this_year() {
    let today = NaiveDate::from_ymd_opt(2023, 4, 23).unwrap();
    let birthday = NaiveDate::from_ymd_opt(1990, 2, 4).unwrap();
    let result = super::days_until_next_birthday(today, birthday).unwrap();

    assert_eq!(result, 287);
  }
}
