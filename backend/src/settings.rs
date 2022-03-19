use crate::errors::ServiceResult;
use config::{Config, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub domain: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,

}

impl Settings {
    pub fn new() -> ServiceResult<Self> {
        let mut s = Config::new();
        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into().map_err(|e| e.into())
    }
}
