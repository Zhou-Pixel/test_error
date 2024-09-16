use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: Uuid,
    pub title: String,
    pub detail: String,
    pub typ: String,
    pub pinned: bool,
    pub created_time: i64,
    pub last_exec_time: Option<i64>,
    pub sub: Vec<u8>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}