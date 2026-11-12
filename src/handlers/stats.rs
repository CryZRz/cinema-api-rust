use crate::{
    error::{AppError, ErrorResponse},
    models::{
        booking::{self, Entity as BookingEntity},
        employee::Entity as EmployeeEntity,
        movie::Entity as MovieEntity,
        room::Entity as RoomEntity,
        screening::Entity as ScreeningEntity,
        stats::CinemaStats,
    },
};
use axum::{extract::State, Json};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[utoipa::path(
    get,
    path = "/api/stats",
    responses(
        (status = 200, description = "Cinema operational statistics", body = CinemaStats),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Statistics"
)]
pub async fn get_cinema_stats(
    State(db): State<DatabaseConnection>,
) -> Result<Json<CinemaStats>, AppError> {
    let all_movies = MovieEntity::find().all(&db).await?;
    let total_movies = all_movies.len() as i64;
    let active_movies = all_movies.iter().filter(|m| m.is_active).count() as i64;

    let all_rooms = RoomEntity::find().all(&db).await?;
    let total_rooms = all_rooms.len() as i64;
    let total_room_capacity: i64 = all_rooms.iter().map(|r| r.capacity as i64).sum();

    let all_employees = EmployeeEntity::find().all(&db).await?;
    let total_employees = all_employees.len() as i64;

    let all_screenings = ScreeningEntity::find().all(&db).await?;
    let total_screenings = all_screenings.len() as i64;

    let all_bookings = BookingEntity::find()
        .filter(booking::Column::Status.eq("CONFIRMED"))
        .all(&db)
        .await?;

    let total_bookings = all_bookings.len() as i64;
    let total_tickets_sold: i64 = all_bookings.iter().map(|b| b.seats_count as i64).sum();
    let total_revenue: Decimal = all_bookings.iter().map(|b| b.total_price).sum();

    Ok(Json(CinemaStats {
        total_movies,
        active_movies,
        total_rooms,
        total_room_capacity,
        total_employees,
        total_screenings,
        total_bookings,
        total_tickets_sold,
        total_revenue,
    }))
}
