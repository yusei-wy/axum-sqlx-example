mod db;
mod entity;
mod handlers;
mod postgres_repository;
mod repository;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use handlers::{all_users, create_user, delete_user, find_user, update_user};
use repository::UserRepository;
use std::{env, net::SocketAddr, sync::Arc};

use crate::{db::create_connection_pool, postgres_repository::PgUserRepository};

fn create_app<T: UserRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(handlers::home))
        .route("/health", get(handlers::health_check))
        .route("/users", post(create_user::<T>).get(all_users::<T>))
        .route(
            "/users/:user_id",
            get(find_user::<T>)
                .patch(update_user::<T>)
                .delete(delete_user::<T>),
        )
        .with_state(Arc::new(repository))
}

#[tokio::main]
async fn main() -> anyhow::Result<(), sqlx::Error> {
    dotenv().ok();

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let pool = create_connection_pool().await;

    let user_repo = PgUserRepository::new(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let app = create_app(user_repo);
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
        let pool = create_connection_pool().await;
        let user_repo = PgUserRepository::new(pool.clone());
        let res = create_app(user_repo).oneshot(req).await.unwrap();
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
        let pool = create_connection_pool().await;
        let user_repo = PgUserRepository::new(pool.clone());
        let res = create_app(user_repo).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");

        assert_eq!(user.nickname, "田中");
        assert_eq!(user.birthday, NaiveDate::from_ymd_opt(1992, 5, 31).unwrap());

        pool.close().await;
    }
}
