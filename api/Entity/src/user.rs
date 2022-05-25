use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

// use crate::db::conn::get_conn;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type="Text")]
    pub username: String,

    #[sea_orm(column_type="Text")]
    pub email: String,

    #[sea_orm(column_type="Text")]
    pub password: String,

    #[sea_orm(column_type="Timestamp")]
    pub created_at: NaiveDateTime,

    #[sea_orm(column_type="Timestamp")]
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation { }

impl RelationTrait for Relation { 
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
