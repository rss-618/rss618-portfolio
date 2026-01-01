use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: env_parse("HOST", "0.0.0.0"),
            port: env_parse("PORT", "3000"),
            cors_origin: env_var("CORS_ORIGIN", "http://localhost:5173"),
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_parse<T: FromStr>(key: &str, default: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    let value = env_var(key, default);
    value.parse().unwrap_or_else(|e| {
        panic!("Failed to parse {key}={value}: {e:?}")
    })
}
