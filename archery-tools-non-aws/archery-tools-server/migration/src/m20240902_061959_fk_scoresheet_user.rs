use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_SCORESHEET_USER: &str = "fk_scoresheet_user";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ScoreSheet::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name(FK_SCORESHEET_USER)
                            .from_tbl(ScoreSheet::Table)
                            .from_col(ScoreSheet::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(ScoreSheet::Table)
                    .name(FK_SCORESHEET_USER)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ScoreSheet {
    #[sea_orm(iden = "scoresheet")]
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "user")]
    Table,
    Id,
}
