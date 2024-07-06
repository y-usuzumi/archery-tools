use std::env;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

const SERVER: &str = "ARCHERY_TOOLS_SERVER";
const POSTGRESQL: &str = "ARCHERY_TOOLS_POSTGRESQL";
const SERVER_DEFAULT: &str = "localhost:9876";
const POSTGRESQL_DEFAULT: &str = "localhost:5432";

#[derive(Deserialize, Debug)]
pub struct Env {
    pub server: String,
    pub postgresql: String,
}

lazy_static! {
    pub static ref ENV: Env = {
        dotenv().ok();
        let server = env::var(SERVER).unwrap_or_else(|_| {
            log::info!("Env var {SERVER} is not set. Using {SERVER_DEFAULT} as default");
            SERVER_DEFAULT.to_string()
        });
        let postgresql = env::var(POSTGRESQL).unwrap_or_else(|_| {
            log::info!("Env var {POSTGRESQL} is not set. Using {POSTGRESQL_DEFAULT} as default");
            POSTGRESQL_DEFAULT.to_string()
        });
        Env { server, postgresql }
    };
}
