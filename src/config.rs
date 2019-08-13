use std::env;

#[derive(Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            username: env::var("LOOPIA_USERNAME").unwrap_or(String::from("")),
            password: env::var("LOOPIA_PASSWORD").unwrap_or(String::from("")),
        }
    }
}
