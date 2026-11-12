use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "employees")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEmployeeRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateEmployeeRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}
