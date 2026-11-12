use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "screenings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub movie_id: i64,
    pub room_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub price: Decimal,
    pub available_seats: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::movie::Entity",
        from = "Column::MovieId",
        to = "super::movie::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Movie,
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Room,
    #[sea_orm(has_many = "super::booking::Entity")]
    Booking,
}

impl Related<super::movie::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movie.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::booking::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Booking.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateScreeningRequest {
    pub movie_id: i64,
    pub room_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub price: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateScreeningRequest {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub price: Option<Decimal>,
    pub available_seats: Option<i32>,
}
