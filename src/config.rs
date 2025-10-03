use crate::error::{Error, Result};
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    // INSTANCE is only visible inside the config function.
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    // || closure to initialize the config only once
    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| { // Fail early if config cannot be loaded.
            panic!("Failed to load config from env: {ex}")
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Self {
            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}