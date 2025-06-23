use std::sync::Arc;
use axum::Extension;
use axum::routing::get;
use sqlx::postgres::PgPoolOptions;

mod endpoints;

pub async fn startup() -> Result<(), Box<dyn std::error::Error>> {

    let db_pool = PgPoolOptions::new()
        .min_connections(16)
        .max_connections(92)
        .test_before_acquire(false)
        .connect("postgres://cars:hellopwd@172.21.194.214/cars?sslmode=require")
        .await?;

    let app = axum::Router::new()
        .merge(endpoints::cars::router())
        .merge(endpoints::prime::router())
        .with_state(Arc::new(db_pool));

    let app_address = &format!("{}:{}", std::net::Ipv4Addr::UNSPECIFIED, 3000);
    let listener = tokio::net::TcpListener::bind(app_address)
        .await?;

    tracing::info!("Listening on {}", app_address);
    axum::serve(listener, app).await?;
    Ok(())
}

