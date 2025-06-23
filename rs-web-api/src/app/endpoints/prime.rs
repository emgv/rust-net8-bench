use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{Json as AxumJson, IntoResponse};

type SqlxPgPool = sqlx::Pool<sqlx::Postgres>;

pub fn router() -> axum::Router<Arc<SqlxPgPool>> {
    axum::Router::new()
        .route("/api/prime/{n}", axum::routing::get(get_prime_entry))
}

async fn get_prime_entry(axum::extract::Path(n): axum::extract::Path<u32>) -> impl IntoResponse {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let mut count = 1u32;
        let mut current_prime = 2u32;
        let mut current_num = 2u32;

        while count < n {
            current_num += 1;

            if is_prime_naive(current_num) {
                current_prime = current_num;
                count += 1;
            }
        }

        send.send(current_prime);
    });

    match recv.await {
        Err(e) => {
            tracing::error!("{:?}", e);
            return super::make_web_api_error_response(StatusCode::INTERNAL_SERVER_ERROR, "An error occurred while getting the prime");
        },
        Ok(nth_prime) => super::make_response_from_object(StatusCode::OK, &nth_prime)
            .unwrap_or_else(|e| super::make_web_api_error_response(StatusCode::INTERNAL_SERVER_ERROR, "An error occurred while creating the response"))
    }
}

fn is_prime_naive(number: u32) -> bool {
    
    if number <= 1 {
        return false;
    }

    if number == 2 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    for i in (3..number).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }

    true
}
