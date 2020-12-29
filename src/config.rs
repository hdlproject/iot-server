pub struct Config {
    pub port: String,
    pub covid19_server_address: String,
}

impl Config {
    pub fn new() -> Config {
        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| String::from("8080")),
            covid19_server_address: std::env::var("COVID19_SERVER_ADDRESS")
                .unwrap_or_else(|_| {
                    println!("COVID19_SERVER_ADDRESS env var is missing");
                    std::process::exit(1)
                }),
        }
    }
}

lazy_static::lazy_static! {
pub static ref CONFIG: Config = Config::new();
}
