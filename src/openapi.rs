use crate::{
    error::ErrorResponse,
    handlers::{booking, employee, health, movie, room, screening, stats},
    models::{
        booking::{AttendeeInfo, CreateBookingRequest, Model as BookingModel},
        employee::{CreateEmployeeRequest, Model as EmployeeModel, UpdateEmployeeRequest},
        movie::{CreateMovieRequest, Model as MovieModel, UpdateMovieRequest},
        room::{CreateRoomRequest, Model as RoomModel, UpdateRoomRequest},
        screening::{CreateScreeningRequest, Model as ScreeningModel, UpdateScreeningRequest},
        stats::CinemaStats,
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        movie::list_movies,
        movie::create_movie,
        movie::get_movie,
        movie::update_movie,
        movie::delete_movie,
        room::list_rooms,
        room::create_room,
        room::get_room,
        room::update_room,
        room::delete_room,
        employee::list_employees,
        employee::create_employee,
        employee::get_employee,
        employee::update_employee,
        employee::delete_employee,
        screening::list_screenings,
        screening::create_screening,
        screening::get_screening,
        screening::update_screening,
        screening::delete_screening,
        screening::list_screening_attendees,
        booking::list_bookings,
        booking::create_booking,
        booking::get_booking,
        booking::cancel_booking,
        stats::get_cinema_stats
    ),
    components(
        schemas(
            MovieModel,
            CreateMovieRequest,
            UpdateMovieRequest,
            RoomModel,
            CreateRoomRequest,
            UpdateRoomRequest,
            EmployeeModel,
            CreateEmployeeRequest,
            UpdateEmployeeRequest,
            ScreeningModel,
            CreateScreeningRequest,
            UpdateScreeningRequest,
            BookingModel,
            CreateBookingRequest,
            AttendeeInfo,
            CinemaStats,
            ErrorResponse
        )
    ),
    tags(
        (name = "Health", description = "System health check"),
        (name = "Movies", description = "Movie catalog management"),
        (name = "Rooms", description = "Cinema hall and auditorium management"),
        (name = "Employees", description = "Staff and employee management"),
        (name = "Screenings", description = "Movie screening schedules and attendee listings"),
        (name = "Bookings", description = "Ticket reservations and seat bookings"),
        (name = "Statistics", description = "Cinema analytics and box office metrics")
    ),
    info(
        title = "Cinema REST API",
        version = "1.0.0",
        description = "Cinema Management REST API built with Rust, Axum, SeaORM, and PostgreSQL"
    )
)]
pub struct ApiDoc;
