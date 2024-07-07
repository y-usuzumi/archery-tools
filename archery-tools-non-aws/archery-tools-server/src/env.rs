use std::env;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

const SERVER: &str = "ARCHERY_TOOLS_SERVER";
const SERVER_DEFAULT: &str = "localhost:9876";

const POSTGRES_HOST: &str = "ARCHERY_TOOLS_POSTGRES_HOST";
const POSTGRES_HOST_DEFAULT: &str = "localhost:5432";

const POSTGRES_USER: &str = "ARCHERY_TOOLS_POSTGRES_USER";
const POSTGRES_USER_DEFAULT: &str = "postgres";

const POSTGRES_PASSWORD: &str = "ARCHERY_TOOLS_POSTGRES_PASSWORD";
const POSTGRES_PASSWORD_DEFAULT: &str = "6sXZQfAPZyGcp7RAsE7u";

const POSTGRES_DB: &str = "ARCHERY_TOOLS_POSTGRES_DB";
const POSTGRES_DB_DEFAULT: &str = "archery-tools";

#[derive(Deserialize, Debug)]
pub struct Env {
    pub server: String,
    pub postgres_host: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
}

fn extract_envvar_with_default(var: &str, default: &str) -> String {
    env::var(var).unwrap_or_else(|_| {
        log::info!("Env var {var} is not set. Using {default} as default");
        default.to_string()
    })
}

lazy_static! {
    pub static ref ENV: Env = {
        dotenv().ok();
        let server = extract_envvar_with_default(SERVER, SERVER_DEFAULT);
        let postgres_host = extract_envvar_with_default(POSTGRES_HOST, POSTGRES_HOST_DEFAULT);
        let postgres_user = extract_envvar_with_default(POSTGRES_USER, POSTGRES_USER_DEFAULT);
        let postgres_password =
            extract_envvar_with_default(POSTGRES_PASSWORD, POSTGRES_PASSWORD_DEFAULT);
        let postgres_db = extract_envvar_with_default(POSTGRES_DB, POSTGRES_DB_DEFAULT);
        let env = Env {
            server,
            postgres_host,
            postgres_user,
            postgres_password,
            postgres_db,
        };
        log::trace!("Env: {:?}", env);
        env
    };
}
