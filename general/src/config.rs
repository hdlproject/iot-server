use std::env;
use std::process;
use lazy_static::lazy_static;

pub struct Config {
    pub port: String,
    pub openai_api_url: String,
    pub openai_api_key: String,
}

impl Config {
    pub fn new() -> Config {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| String::from("8080")),
            openai_api_url: env::var("OPENAI_API_URL")
                .unwrap_or_else(|e| {
                    println!("OPENAI_API_URL env var is missing {}", e);
                    process::exit(1)
                }),
            openai_api_key: env::var("OPENAI_API_KEY")
                .unwrap_or_else(|e| {
                    println!("OPENAI_API_KEY env var is missing {}", e);
                    process::exit(1)
                }),
        }
    }
}

lazy_static! {
pub static ref CONFIG: Config = Config::new();
}
