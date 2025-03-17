use crate::errors::Error;
use crate::Result;
use std::env;
use std::str::FromStr;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use dotenvy::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub port: u16,
    pub counter: Arc<AtomicU64>,
    pub work_dir: String,
}

impl AppState {
    pub fn new() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            port: get_env_as_parse("SERVICE_PORT")?,
            counter: Arc::new(AtomicU64::new(get_env_as_parse("START_COUNTER")?)),
            work_dir: get_env("WORK_DIR")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_as_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}