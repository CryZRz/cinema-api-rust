use crate::{
    error::{AppError, ErrorResponse},
    models::{
        booking::{self, AttendeeInfo, Entity as BookingEntity},
        movie::Entity as MovieEntity,
        room::Entity as RoomEntity,
        screening::{self, ActiveModel, CreateScreeningRequest, Entity as ScreeningEntity, UpdateScreeningRequest},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

#[utoipa::path(
    get,
    path = "/api/screenings",
    responses(
        (status = 200, description = "List all screenings", body = Vec<screening::Model>),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Screenings"
)]
pub async fn list_screenings(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<screening::Model>>, AppError> {
    let screenings = ScreeningEntity::find().all(&db).await?;
    Ok(Json(screenings))
}

#[utoipa::path(
    post,
    path = "/api/screenings",
    request_body = CreateScreeningRequest,
    responses(
        (status = 201, description = "Screening scheduled successfully", body = screening::Model),
        (status = 400, description = "Invalid movie or room reference", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Screenings"
)]
pub async fn create_screening(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateScreeningRequest>,
) -> Result<(StatusCode, Json<screening::Model>), AppError> {
    let _movie = MovieEntity::find_by_id(payload.movie_id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::BadRequest(format!("Movie with id {} does not exist", payload.movie_id)))?;

    let room = RoomEntity::find_by_id(payload.room_id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::BadRequest(format!("Room with id {} does not exist", payload.room_id)))?;

    if payload.end_time <= payload.start_time {
        return Err(AppError::BadRequest("End time must be after start time".to_string()));
    }

    let active_screening = ActiveModel {
        movie_id: Set(payload.movie_id),
        room_id: Set(payload.room_id),
        start_time: Set(payload.start_time),
        end_time: Set(payload.end_time),
        price: Set(payload.price),
        available_seats: Set(room.capacity),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let screening = active_screening.insert(&db).await?;
    Ok((StatusCode::CREATED, Json(screening)))
}

#[utoipa::path(
    get,
    path = "/api/screenings/{id}",
    params(
        ("id" = i64, Path, description = "Screening identifier")
    ),
    responses(
        (status = 200, description = "Screening found", body = screening::Model),
        (status = 404, description = "Screening not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Screenings"
)]
pub async fn get_screening(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<screening::Model>, AppError> {
    let screening = ScreeningEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Screening with id {} not found", id)))?;

    Ok(Json(screening))
}

#[utoipa::path(
    put,
    path = "/api/screenings/{id}",
    params(
        ("id" = i64, Path, description = "Screening identifier")
    ),
    request_body = UpdateScreeningRequest,
    responses(
        (status = 200, description = "Screening updated successfully", body = screening::Model),
        (status = 404, description = "Screening not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Screenings"
)]
pub async fn update_screening(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateScreeningRequest>,
) -> Result<Json<screening::Model>, AppError> {
    let screening = ScreeningEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Screening with id {} not found", id)))?;

    let mut active: ActiveModel = screening.into_active_model();

    if let Some(start_time) = payload.start_time {
        active.start_time = Set(start_time);
    }
    if let Some(end_time) = payload.end_time {
        active.end_time = Set(end_time);
    }
    if let Some(price) = payload.price {
        active.price = Set(price);
    }
    if let Some(available_seats) = payload.available_seats {
        active.available_seats = Set(available_seats);
    }

    let updated = active.update(&db).await?;
    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/api/screenings/{id}",
    params(
        ("id" = i64, Path, description = "Screening identifier")
    ),
    responses(
        (status = 204, description = "Screening deleted successfully"),
        (status = 404, description = "Screening not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Screenings"
)]
pub async fn delete_screening(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let screening = ScreeningEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Screening with id {} not found", id)))?;

    let active: ActiveModel = screening.into_active_model();
    active.delete(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/screenings/{id}/attendees",
    params(
        ("id" = i64, Path, description = "Screening identifier")
    ),
    responses(
        (status = 200, description = "List of attendees for screening", body = Vec<AttendeeInfo>),
        (status = 404, description = "Screening not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Screenings"
)]
pub async fn list_screening_attendees(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<AttendeeInfo>>, AppError> {
    let _screening = ScreeningEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Screening with id {} not found", id)))?;

    let bookings = BookingEntity::find()
        .filter(booking::Column::ScreeningId.eq(id))
        .all(&db)
        .await?;

    let attendees = bookings
        .into_iter()
        .map(|b| AttendeeInfo {
            booking_id: b.id,
            customer_name: b.customer_name,
            customer_email: b.customer_email,
            seats_count: b.seats_count,
            status: b.status,
            created_at: b.created_at,
        })
        .collect();

    Ok(Json(attendees))
}
