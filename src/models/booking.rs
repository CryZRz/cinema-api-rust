use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "bookings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub screening_id: i64,
    pub customer_name: String,
    pub customer_email: String,
    pub seats_count: i32,
    pub total_price: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::screening::Entity",
        from = "Column::ScreeningId",
        to = "super::screening::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Screening,
}

impl Related<super::screening::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Screening.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBookingRequest {
    pub screening_id: i64,
    pub customer_name: String,
    pub customer_email: String,
    pub seats_count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AttendeeInfo {
    pub booking_id: i64,
    pub customer_name: String,
    pub customer_email: String,
    pub seats_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
