use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(ScoreSheet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScoreSheet::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ScoreSheet::UserId).integer().not_null())
                    .col(ColumnDef::new(ScoreSheet::DateTime).string().not_null())
                    .to_owned(),
            )
            .await;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("idx_scoresheet_user_id")
                    .table(ScoreSheet::Table)
                    .if_not_exists()
                    .col(ScoreSheet::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ScoreSheet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ScoreSheet {
    #[sea_orm(iden = "scoresheet")]
    Table,
    Id,
    UserId,
    DateTime,
}
