use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CinemaStats {
    pub total_movies: i64,
    pub active_movies: i64,
    pub total_rooms: i64,
    pub total_room_capacity: i64,
    pub total_employees: i64,
    pub total_screenings: i64,
    pub total_bookings: i64,
    pub total_tickets_sold: i64,
    pub total_revenue: Decimal,
}
