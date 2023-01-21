use std::env;

pub struct Config {
  pub port: u16,
  pub allowed_origin: String,
  pub firestore_project: String,
}

impl Config {
  pub fn from_env() -> Config {
    Config {
      port: env::var("PORT")
        .unwrap_or_else(|_| String::from("8000"))
        .parse()
        .expect("PORT to be an integer"),
      allowed_origin: env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN environment variable"),
      firestore_project: env::var("FIRESTORE_PROJECT")
        .expect("FIRESTORE_PROJECT environment variable"),
    }
  }
}

impl Clone for Config {
  fn clone(&self) -> Config {
    Config {
      port: self.port,
      allowed_origin: self.allowed_origin.clone(),
      firestore_project: self.firestore_project.clone(),
    }
  }
}
