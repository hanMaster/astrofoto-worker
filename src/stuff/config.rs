use crate::errors::Error;
use crate::Result;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            panic!("FATAL - WHILE LOADING Config -cause: {:?}", err);
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub SERVICE_PORT: u16,
    pub WORK_DIR: String,
    // -- Init value for order folder name
    pub START_COUNTER: u64,
    // -- Mailer
    pub SMTP_SERVER: String,
    pub SENDER_EMAIL: String,
    pub SENDER_PASS: String,
    pub RECEIVER_EMAIL: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        dotenv()?;
        Ok(Config {
            SERVICE_PORT: get_env_as_parse("SERVICE_PORT")?,
            WORK_DIR: get_env("WORK_DIR")?,
            START_COUNTER: get_env_as_parse("START_COUNTER")?,
            SMTP_SERVER: get_env("SMTP_SERVER")?,
            SENDER_EMAIL: get_env("SENDER_EMAIL")?,
            SENDER_PASS: get_env("SENDER_PASS")?,
            RECEIVER_EMAIL: get_env("RECEIVER_EMAIL")?,
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
