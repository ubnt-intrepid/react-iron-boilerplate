use std::env;

pub struct AppConfig {
    pub bind_address: String,
}

impl AppConfig {
    pub fn from_env_vars() -> AppConfig {
        let bind_address = env::var("BIND_ADDR").unwrap_or("0.0.0.0:3000".to_owned());
        AppConfig { bind_address: bind_address }
    }
}
