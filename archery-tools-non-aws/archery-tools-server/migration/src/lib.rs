pub use sea_orm_migration::prelude::*;

mod m20240902_060944_user;
mod m20240902_061310_scoresheet;
mod m20240902_061959_fk_scoresheet_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240902_060944_user::Migration),
            Box::new(m20240902_061310_scoresheet::Migration),
            Box::new(m20240902_061959_fk_scoresheet_user::Migration),
        ]
    }
}
