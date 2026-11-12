use cinema_api_rust::models::movie::CreateMovieRequest;
use cinema_api_rust::models::room::CreateRoomRequest;
use cinema_api_rust::models::stats::CinemaStats;
use rust_decimal::Decimal;

#[test]
fn test_create_movie_request_serialization() {
    let req = CreateMovieRequest {
        title: "Interstellar".to_string(),
        synopsis: "Explorers travel through a wormhole in space".to_string(),
        duration_minutes: 169,
        genre: "Sci-Fi".to_string(),
        rating: "PG-13".to_string(),
    };
    let json = serde_json::to_string(&req).expect("Failed to serialize");
    assert!(json.contains("Interstellar"));
}

#[test]
fn test_create_room_request_serialization() {
    let req = CreateRoomRequest {
        name: "Sala 3".to_string(),
        capacity: 100,
        room_type: "3D".to_string(),
    };
    let json = serde_json::to_string(&req).expect("Failed to serialize");
    assert!(json.contains("Sala 3"));
}

#[test]
fn test_stats_initialization() {
    let stats = CinemaStats {
        total_movies: 5,
        active_movies: 4,
        total_rooms: 2,
        total_room_capacity: 150,
        total_employees: 3,
        total_screenings: 8,
        total_bookings: 10,
        total_tickets_sold: 25,
        total_revenue: Decimal::new(3000, 0),
    };
    assert_eq!(stats.total_movies, 5);
    assert_eq!(stats.total_tickets_sold, 25);
}
