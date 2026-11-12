use crate::{
    error::{AppError, ErrorResponse},
    models::{
        booking::{self, ActiveModel as BookingActiveModel, CreateBookingRequest, Entity as BookingEntity},
        screening::{ActiveModel as ScreeningActiveModel, Entity as ScreeningEntity},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set, TransactionTrait,
};

#[utoipa::path(
    get,
    path = "/api/bookings",
    responses(
        (status = 200, description = "List all bookings", body = Vec<booking::Model>),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Bookings"
)]
pub async fn list_bookings(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<booking::Model>>, AppError> {
    let bookings = BookingEntity::find().all(&db).await?;
    Ok(Json(bookings))
}

#[utoipa::path(
    post,
    path = "/api/bookings",
    request_body = CreateBookingRequest,
    responses(
        (status = 201, description = "Booking created successfully", body = booking::Model),
        (status = 400, description = "Insufficient seats or invalid parameters", body = ErrorResponse),
        (status = 404, description = "Screening not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Bookings"
)]
pub async fn create_booking(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateBookingRequest>,
) -> Result<(StatusCode, Json<booking::Model>), AppError> {
    if payload.seats_count <= 0 {
        return Err(AppError::BadRequest("Seats count must be greater than zero".to_string()));
    }

    let txn = db.begin().await?;

    let screening = ScreeningEntity::find_by_id(payload.screening_id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Screening with id {} not found", payload.screening_id)))?;

    if screening.available_seats < payload.seats_count {
        return Err(AppError::BadRequest(format!(
            "Not enough seats available. Requested: {}, Available: {}",
            payload.seats_count, screening.available_seats
        )));
    }

    let seat_multiplier = Decimal::from(payload.seats_count);
    let total_price = screening.price * seat_multiplier;

    let mut screening_active: ScreeningActiveModel = screening.into_active_model();
    screening_active.available_seats = Set(screening_active.available_seats.unwrap() - payload.seats_count);
    screening_active.update(&txn).await?;

    let booking_active = BookingActiveModel {
        screening_id: Set(payload.screening_id),
        customer_name: Set(payload.customer_name),
        customer_email: Set(payload.customer_email),
        seats_count: Set(payload.seats_count),
        total_price: Set(total_price),
        status: Set("CONFIRMED".to_string()),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let booking = booking_active.insert(&txn).await?;
    txn.commit().await?;

    Ok((StatusCode::CREATED, Json(booking)))
}

#[utoipa::path(
    get,
    path = "/api/bookings/{id}",
    params(
        ("id" = i64, Path, description = "Booking identifier")
    ),
    responses(
        (status = 200, description = "Booking found", body = booking::Model),
        (status = 404, description = "Booking not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Bookings"
)]
pub async fn get_booking(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<booking::Model>, AppError> {
    let booking = BookingEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Booking with id {} not found", id)))?;

    Ok(Json(booking))
}

#[utoipa::path(
    post,
    path = "/api/bookings/{id}/cancel",
    params(
        ("id" = i64, Path, description = "Booking identifier")
    ),
    responses(
        (status = 200, description = "Booking cancelled successfully", body = booking::Model),
        (status = 400, description = "Booking is already cancelled", body = ErrorResponse),
        (status = 404, description = "Booking not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Bookings"
)]
pub async fn cancel_booking(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<booking::Model>, AppError> {
    let txn = db.begin().await?;

    let booking = BookingEntity::find_by_id(id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Booking with id {} not found", id)))?;

    if booking.status == "CANCELLED" {
        return Err(AppError::BadRequest("Booking is already cancelled".to_string()));
    }

    if let Some(screening) = ScreeningEntity::find_by_id(booking.screening_id).one(&txn).await? {
        let mut screening_active: ScreeningActiveModel = screening.into_active_model();
        let new_seats = screening_active.available_seats.unwrap() + booking.seats_count;
        screening_active.available_seats = Set(new_seats);
        screening_active.update(&txn).await?;
    }

    let mut booking_active: BookingActiveModel = booking.into_active_model();
    booking_active.status = Set("CANCELLED".to_string());
    let updated_booking = booking_active.update(&txn).await?;

    txn.commit().await?;
    Ok(Json(updated_booking))
}
