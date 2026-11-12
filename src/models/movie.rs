use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "movies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub synopsis: String,
    pub duration_minutes: i32,
    pub genre: String,
    pub rating: String,
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
pub struct CreateMovieRequest {
    pub title: String,
    pub synopsis: String,
    pub duration_minutes: i32,
    pub genre: String,
    pub rating: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateMovieRequest {
    pub title: Option<String>,
    pub synopsis: Option<String>,
    pub duration_minutes: Option<i32>,
    pub genre: Option<String>,
    pub rating: Option<String>,
    pub is_active: Option<bool>,
}
