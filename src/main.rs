use cinema_api_rust::{config, db, routes};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cinema_api_rust=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env();

    tracing::info!("Connecting to PostgreSQL database");
    let db = db::connect(&cfg.database_url).await?;

    tracing::info!("Initializing database schema");
    db::initialize_schema(&db).await?;

    let app = routes::create_router(db);

    let addr: SocketAddr = format!("{}:{}", cfg.server_host, cfg.server_port).parse()?;
    tracing::info!("Cinema REST API server listening on {}", addr);
    tracing::info!("Swagger UI documentation available at http://{}/swagger-ui", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
