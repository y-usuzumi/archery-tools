use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "scoresheet")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub archer_id: Uuid,
    pub datetime: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(has_many = "super::fruit::Entity")]
    // Fruit,
    
}

impl ActiveModelBehavior for ActiveModel {}
