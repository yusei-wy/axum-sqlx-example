use anyhow::Result;
use axum::async_trait;
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    entity::{CreateUserPayload, UpdateUserPayload, User},
    repository::{self, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> PgUserRepository {
        PgUserRepository { pool }
    }
}

#[async_trait]
impl repository::UserRepository for PgUserRepository {
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

    async fn find(&self, user_id: Uuid) -> Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn update(&self, user_id: Uuid, payload: UpdateUserPayload) -> Result<User> {
        let user = self.find(user_id).await?;

        let birthday = NaiveDate::parse_from_str(&payload.birthday, "%Y-%m-%d").unwrap();

        let result_user = sqlx::query_as!(
            User,
            r#"
UPDATE users SET nickname = $1, birthday = $2 WHERE user_id = $3
RETURNING *
"#,
            payload.nickname,
            birthday,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result_user)
    }

    async fn delete(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM users WHERE user_id = $"#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(user_id.to_string()),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}
