use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::{self, Connection, PgConnection};
use std::{env, net::SocketAddr, sync::Arc};
use uuid::Uuid;

#[derive(Serialize)]
struct User {
    user_id: Uuid,
    nickname: String,
    birthday: NaiveDate,
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

async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

async fn home() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let conn = PgConnection::connect(&db_url).await.unwrap();
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health_check))
        .layer(Extension(Arc::new(conn)));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
