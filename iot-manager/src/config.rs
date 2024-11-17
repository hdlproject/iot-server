use std::env;
use std::process;
use lazy_static::lazy_static;

pub struct Config {
    pub port: String,
    pub postgres_url: String,
    pub general_service_url: String,
}

impl Config {
    pub fn new() -> Config {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| String::from("8082")),
            postgres_url: env::var("POSTGRES_URL")
                .unwrap_or_else(|e| {
                    println!("POSTGRES_URL env var is missing {}", e);
                    process::exit(1)
                }),
            general_service_url: env::var("GENERAL_SERVICE_URL")
                .unwrap_or_else(|_| String::from("localhost:8081")),
        }
    }
}

lazy_static! {
pub static ref CONFIG: Config = Config::new();
}
