use anyhow::Result;
use axum::async_trait;
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    entity::{CreateUserPayload, UpdateUserPayload, User},
    repository,
};

pub struct Repositories {
    pub user_repository: UserRepository,
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> UserRepository {
        UserRepository { pool }
    }
}

#[async_trait]
impl repository::UserRepository for UserRepository {
    async fn create(&self, payload: CreateUserPayload) -> Result<User> {
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
        .execute(&self.pool)
        .await
        .ok();

        Ok(result_user)
    }

    async fn find(&self, user_id: Uuid) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id)
            .fetch_one(&self.pool)
            .await
            .ok()
    }

    async fn all(&self) -> Vec<User> {
        sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .ok()
            .map_or_else(|| vec![], |u| u)
    }

    async fn update(&self, payload: UpdateUserPayload) -> Result<User> {
        todo!()
        // let birthday = NaiveDate::parse_from_str(&payload.birthday, "%Y-%m-%d").unwrap();

        // let user = self.find(payload.user_id);
        // let updated_user = {
        //     ...user,
        //     nickname: payload.nickname,
        //     birthday,
        // };

        // sqlx::query(
        //     r#"
        // UPDATE users SET nickname = $1, birthday = $2 WHERE user_id = $3
        // "#,
        // )
        // .bind(payload.nickname)
        // .bind(birthday)
        // .bind(payload.user_id)
        // .execute(&pool)
        // .await
        // .ok();
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        todo!();
    }
}
