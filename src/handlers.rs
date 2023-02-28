use std::sync::Arc;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path},
    http::request::Parts,
    response::IntoResponse,
    BoxError, Json,
};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use sqlx::types::uuid::Uuid;
use validator::Validate;

use crate::{
    entity::{CreateUserPayload, UpdateUserPayload},
    repository::UserRepository,
};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut Parts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
            let message = format!("Json parse error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn home() -> &'static str {
    "Hello World"
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub async fn create_user<T: UserRepository>(
    ValidatedJson(payload): Json<CreateUserPayload>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn all_users<T: UserRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let users = repository.all().await.or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(users)))
}

pub async fn find_user<T: UserRepository>(
    Path(user_id): Path<Uuid>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .find(user_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user<T: UserRepository>(
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = repository
        .update(user_id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user<T: UserRepository>(
    Path(user_id): Path<Uuid>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    repository
        .delete(user_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(StatusCode::OK)
}
