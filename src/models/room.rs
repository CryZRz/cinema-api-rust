use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub capacity: i32,
    pub room_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::screening::Entity")]
    Screening,
}

impl Related<super::screening::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Screening.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateRoomRequest {
    pub name: String,
    pub capacity: i32,
    pub room_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub capacity: Option<i32>,
    pub room_type: Option<String>,
    pub is_active: Option<bool>,
}
