use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use sqlx::types::uuid::Uuid;

use crate::{
    entity::{CreateUserPayload, UpdateUserPayload},
    repository::UserRepository,
};

pub async fn home() -> &'static str {
    "Hello World"
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub async fn create_user<T: UserRepository>(
    State(repository): State<Arc<T>>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn all_users<T: UserRepository>(
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let users = repository.all().await.or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(users)))
}

pub async fn find_user<T: UserRepository>(
    State(repository): State<Arc<T>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .find(user_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user<T: UserRepository>(
    State(repository): State<Arc<T>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .update(user_id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user<T: UserRepository>(
    State(repository): State<Arc<T>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    repository
        .delete(user_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(StatusCode::OK)
}
