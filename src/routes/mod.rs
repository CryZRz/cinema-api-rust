use crate::{
    handlers::{booking, employee, health, movie, room, screening, stats},
    openapi::ApiDoc,
};
use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/movies", get(movie::list_movies).post(movie::create_movie))
        .route(
            "/movies/{id}",
            get(movie::get_movie)
                .put(movie::update_movie)
                .delete(movie::delete_movie),
        )
        .route("/rooms", get(room::list_rooms).post(room::create_room))
        .route(
            "/rooms/{id}",
            get(room::get_room)
                .put(room::update_room)
                .delete(room::delete_room),
        )
        .route(
            "/employees",
            get(employee::list_employees).post(employee::create_employee),
        )
        .route(
            "/employees/{id}",
            get(employee::get_employee)
                .put(employee::update_employee)
                .delete(employee::delete_employee),
        )
        .route(
            "/screenings",
            get(screening::list_screenings).post(screening::create_screening),
        )
        .route(
            "/screenings/{id}",
            get(screening::get_screening)
                .put(screening::update_screening)
                .delete(screening::delete_screening),
        )
        .route(
            "/screenings/{id}/attendees",
            get(screening::list_screening_attendees),
        )
        .route(
            "/bookings",
            get(booking::list_bookings).post(booking::create_booking),
        )
        .route("/bookings/{id}", get(booking::get_booking))
        .route("/bookings/{id}/cancel", post(booking::cancel_booking))
        .route("/stats", get(stats::get_cinema_stats));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health::health_check))
        .nest("/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(db)
}
