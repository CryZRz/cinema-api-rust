use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr, Statement,
};
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(50)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false);

    Database::connect(opt).await
}

pub async fn initialize_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = DatabaseBackend::Postgres;

    let ddl_statements = [
        "CREATE TABLE IF NOT EXISTS movies (
            id BIGSERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            synopsis TEXT NOT NULL,
            duration_minutes INTEGER NOT NULL,
            genre VARCHAR(100) NOT NULL,
            rating VARCHAR(20) NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );",
        "CREATE TABLE IF NOT EXISTS rooms (
            id BIGSERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL UNIQUE,
            capacity INTEGER NOT NULL,
            room_type VARCHAR(50) NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );",
        "CREATE TABLE IF NOT EXISTS employees (
            id BIGSERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            phone VARCHAR(50) NOT NULL,
            role VARCHAR(50) NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );",
        "CREATE TABLE IF NOT EXISTS screenings (
            id BIGSERIAL PRIMARY KEY,
            movie_id BIGINT NOT NULL REFERENCES movies(id) ON DELETE CASCADE,
            room_id BIGINT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
            start_time TIMESTAMPTZ NOT NULL,
            end_time TIMESTAMPTZ NOT NULL,
            price NUMERIC(10,2) NOT NULL,
            available_seats INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );",
        "CREATE TABLE IF NOT EXISTS bookings (
            id BIGSERIAL PRIMARY KEY,
            screening_id BIGINT NOT NULL REFERENCES screenings(id) ON DELETE CASCADE,
            customer_name VARCHAR(255) NOT NULL,
            customer_email VARCHAR(255) NOT NULL,
            seats_count INTEGER NOT NULL,
            total_price NUMERIC(10,2) NOT NULL,
            status VARCHAR(50) NOT NULL DEFAULT 'CONFIRMED',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );",
    ];

    for stmt_str in ddl_statements {
        let stmt = Statement::from_string(backend, stmt_str.to_string());
        db.execute(stmt).await?;
    }

    Ok(())
}
