mod entity;
mod handlers;
mod postgres_repository;
mod repository;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr};

async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(handlers::home))
        .route("/health", get(handlers::health_check))
        .nest(
            "/api",
            Router::new().nest(
                "/users",
                Router::new()
                    .route("/", get(handlers::all_users))
                    .route("/", post(handlers::create_user))
                    .route("/:user_id", get(handlers::find_user)),
            ),
        )
        .with_state(pool)
}

#[tokio::main]
async fn main() -> anyhow::Result<(), sqlx::Error> {
    dotenv().ok();

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let pool = create_pool().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let app = create_app(pool.clone());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    pool.close().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::entity::User;

    use super::*;

    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use chrono::NaiveDate;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        dotenv().ok();

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let pool = create_pool().await.unwrap();
        let res = create_app(pool.clone()).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello World");

        pool.close().await;
    }

    #[tokio::test]
    async fn should_return_user_data() {
        dotenv().ok();

        let req = Request::builder()
            .uri("/api/users/")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                r#"{"nickname": "田中", "birthday": "1992-05-31"}"#,
            ))
            .unwrap();
        let pool = create_pool().await.unwrap();
        let res = create_app(pool.clone()).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");

        assert_eq!(user.nickname, "田中");
        assert_eq!(user.birthday, NaiveDate::from_ymd_opt(1992, 5, 31).unwrap());

        pool.close().await;
    }
}
