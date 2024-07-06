use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::env::ENV;

const DB_SCHEMA: &str = "main";

fn get_connection_string() -> String {
    format!(
        "postgres://{user}:{password}@{host}/{db}?currentSchema={schema}",
        user = ENV.postgres_user,
        password = ENV.postgres_password,
        host = ENV.postgres_password,
        db = ENV.postgres_db,
        schema = DB_SCHEMA
    )
}

pub(crate) async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect(get_connection_string()).await
}
