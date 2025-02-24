use std::env;

pub struct Config {
    pub ip_address: String,
    pub port: u16,
    pub session_key: String,
    pub mongo_connection: String,
}

impl Config {
    pub fn build() -> Self {
        Self {
            ip_address: env::var("IP_ADDRESS").unwrap_or(String::from("127.0.0.1")),
            port: env::var("PORT")
                .unwrap_or(String::from("3030"))
                .parse()
                .unwrap(),
            session_key: env::var("SESSION_KEY").expect("SESSION_KEY env variable must be set"),
            mongo_connection: env::var("MONGO_CONNECTION")
                .expect("MONGO_CONNECTION env variable must be set"),
        }
    }
}
