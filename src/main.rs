use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::uuid::Uuid, PgPool};
use std::{env, net::SocketAddr};

#[derive(Serialize, Clone, Debug)]
struct User {
    user_id: Uuid,
    nickname: String,
    birthday: NaiveDate,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateUserInput {
    nickname: String,
    birthday: String,
}

#[derive(Serialize, Debug)]
struct Todo {
    todo_id_: Uuid,
    user_id: Uuid,
    title: String,
    status: String,
    published_at: NaiveDateTime,
    edited_at: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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
        .with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    pool.close().await;
    Ok(())
}
