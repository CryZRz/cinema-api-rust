use crate::{
    error::{AppError, ErrorResponse},
    models::room::{self, ActiveModel, CreateRoomRequest, Entity as RoomEntity, UpdateRoomRequest},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

#[utoipa::path(
    get,
    path = "/api/rooms",
    responses(
        (status = 200, description = "List all cinema rooms", body = Vec<room::Model>),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Rooms"
)]
pub async fn list_rooms(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<room::Model>>, AppError> {
    let rooms = RoomEntity::find().all(&db).await?;
    Ok(Json(rooms))
}

#[utoipa::path(
    post,
    path = "/api/rooms",
    request_body = CreateRoomRequest,
    responses(
        (status = 201, description = "Cinema room created successfully", body = room::Model),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Rooms"
)]
pub async fn create_room(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<(StatusCode, Json<room::Model>), AppError> {
    let active_room = ActiveModel {
        name: Set(payload.name),
        capacity: Set(payload.capacity),
        room_type: Set(payload.room_type),
        is_active: Set(true),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let room = active_room.insert(&db).await?;
    Ok((StatusCode::CREATED, Json(room)))
}

#[utoipa::path(
    get,
    path = "/api/rooms/{id}",
    params(
        ("id" = i64, Path, description = "Room identifier")
    ),
    responses(
        (status = 200, description = "Cinema room found", body = room::Model),
        (status = 404, description = "Cinema room not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Rooms"
)]
pub async fn get_room(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<room::Model>, AppError> {
    let room = RoomEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Room with id {} not found", id)))?;

    Ok(Json(room))
}

#[utoipa::path(
    put,
    path = "/api/rooms/{id}",
    params(
        ("id" = i64, Path, description = "Room identifier")
    ),
    request_body = UpdateRoomRequest,
    responses(
        (status = 200, description = "Cinema room updated successfully", body = room::Model),
        (status = 404, description = "Cinema room not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Rooms"
)]
pub async fn update_room(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRoomRequest>,
) -> Result<Json<room::Model>, AppError> {
    let room = RoomEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Room with id {} not found", id)))?;

    let mut active: ActiveModel = room.into_active_model();

    if let Some(name) = payload.name {
        active.name = Set(name);
    }
    if let Some(capacity) = payload.capacity {
        active.capacity = Set(capacity);
    }
    if let Some(room_type) = payload.room_type {
        active.room_type = Set(room_type);
    }
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }

    let updated = active.update(&db).await?;
    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/api/rooms/{id}",
    params(
        ("id" = i64, Path, description = "Room identifier")
    ),
    responses(
        (status = 204, description = "Cinema room deleted successfully"),
        (status = 404, description = "Cinema room not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Rooms"
)]
pub async fn delete_room(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let room = RoomEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Room with id {} not found", id)))?;

    let active: ActiveModel = room.into_active_model();
    active.delete(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}
