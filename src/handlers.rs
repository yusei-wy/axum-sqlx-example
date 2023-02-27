use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDate, Utc};
use hyper::StatusCode;
use sqlx::{types::uuid::Uuid, PgPool};

use crate::entity::{CreateUserPayload, UpdateUserPayload, User};

pub async fn home() -> &'static str {
    "Hello World"
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub async fn create_user(
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

pub async fn all_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .ok();

    (StatusCode::OK, Json(users))
}

pub async fn find_user(State(pool): State<PgPool>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
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

pub fn update_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
    // let birthday = NaiveDate::parse_from_str(&payload.birthday, "%Y-%m-%d").unwrap();

    // let updated_user =

    // sqlx::query(
    //     r#"
    //     UPDATE users SET nickname = $1, birthday = $2 WHERE user_id = $3
    //     "#,
    // )
    // .bind(payload.nickname)
    // .bind(birthday)
    // .bind(payload.user_id)
    // .execute(&pool)
    // .await
    // .ok();

    // (StatusCode::OK, Json(result_user))

    todo!()
}
