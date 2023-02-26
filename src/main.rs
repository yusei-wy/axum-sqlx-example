use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::uuid::Uuid, PgPool};
use std::{env, net::SocketAddr};

#[derive(Serialize, Clone)]
struct User {
    user_id: Uuid,
    nickname: String,
    birthday: NaiveDate,
}

#[derive(Deserialize)]
struct CreateUserInput {
    nickname: String,
    birthday: String,
}

#[derive(Serialize)]
struct Todo {
    todo_id_: Uuid,
    user_id: Uuid,
    title: String,
    status: String,
    published_at: NaiveDateTime,
    edited_at: NaiveDateTime,
}

async fn home() -> &'static str {
    "Hello World"
}

async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserInput>,
) -> (StatusCode, Json<User>) {
    let birthday = NaiveDate::parse_from_str(&payload.birthday, "%Y-%m-%d").unwrap();

    let new_user = User {
        user_id: Uuid::new_v4(),
        nickname: payload.nickname,
        birthday,
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

#[tokio::main]
async fn main() -> anyhow::Result<(), sqlx::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health_check))
        .route("/api/users", post(create_user))
        .with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    pool.close().await;
    Ok(())
}
