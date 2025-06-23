use std::sync::Arc;
use axum::response::{Json as AxumJson, IntoResponse};
use super::WebApiError;
use uuid::Uuid;
use axum::extract::State as AxumState;
use axum::extract::Path as AxumPath;
use axum::http::StatusCode;
use serde::Serialize;

type SqlxPgPool = sqlx::Pool<sqlx::Postgres>;
type AxumResponse = axum::response::Response<axum::body::Body>;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Car {
    pub id: i64,
    pub year: i16,
    pub make: String,
    pub model: String,
}

pub fn router() -> axum::Router<Arc<SqlxPgPool>> {
    axum::Router::new()
        .route("/api/cars-sqlx/{count}", axum::routing::get(get_cars_sqlx_entry))
}

async fn get_cars_sqlx_entry(
    AxumPath(count): AxumPath<i32>,
    AxumState(pg_pool): AxumState<Arc<SqlxPgPool>>
) -> impl IntoResponse {
    match get_cars_sqlx(pg_pool.as_ref(), count).await {
        Err(e) => {
            tracing::error!("{:?}", e);
            super::make_web_api_error_response(StatusCode::INTERNAL_SERVER_ERROR, "An error occurred while getting the cars")
        },
        Ok(items) => super::make_response_from_object(StatusCode::OK, &items)
            .unwrap_or_else(|e| super::make_web_api_error_response(StatusCode::INTERNAL_SERVER_ERROR, "An error occurred while creating the response"))
    }
}

async fn get_cars_sqlx(pg_pool: &SqlxPgPool, count: i32) -> Result<Vec<Car>, Box<dyn std::error::Error>> {

    let cars: Vec<Car> = sqlx::query_as("select * from car order by RANDOM() limit $1")
        .bind(&count)
        .fetch_all(pg_pool)
        .await?;

    Ok(cars)
}
