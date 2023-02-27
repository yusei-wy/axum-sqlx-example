mod entity;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{NaiveDate, Utc};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, types::uuid::Uuid, PgPool};
use std::{env, net::SocketAddr};

use entity::{CreateUserPayload, Todo, User};

async fn home() -> &'static str {
    "Hello World"
}

async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let birthday = NaiveDate::parse_from_str(&payload.birthday, "%Y-%m-%d").unwrap();

    let new_user = User {
        user_id: Uuid::new_v4(),
        nickname: payload.nickname,
        birthday,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let result_user = new_user.clone();

    sqlx::query(
        r#"
        INSERT INTO users (user_id, nickname, birthday) VALUES ($1, $2, $3)
        "#,
    )
    .bind(new_user.user_id)
    .bind(new_user.nickname)
    .bind(new_user.birthday)
    .execute(&pool)
    .await
    .ok();

    (StatusCode::CREATED, Json(result_user))
}

async fn all_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .ok();

    (StatusCode::OK, Json(users))
}

async fn find_user(State(pool): State<PgPool>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id,)
        .fetch_one(&pool)
        .await
        .ok();
    match user {
        Some(user) => (StatusCode::OK, Json(user)),
        // TODO: 見つからなかった場合 dummy user を返さないように
        None => (
            StatusCode::NOT_FOUND,
            Json(User {
                user_id: Uuid::new_v4(),
                nickname: "dummy".to_string(),
                birthday: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }),
        ),
    }
}

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
        .route("/", get(home))
        .route("/health", get(health_check))
        .nest(
            "/api",
            Router::new().nest(
                "/users",
                Router::new()
                    .route("/", get(all_users))
                    .route("/", post(create_user))
                    .route("/:user_id", get(find_user)),
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
    use super::*;

    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
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
