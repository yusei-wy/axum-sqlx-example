use anyhow::Result;
use axum::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::entity::{
    CreateTodoPayload, CreateUserPayload, Todo, UpdateTodoPayload, UpdateUserPayload, User,
};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(String),
}

#[async_trait]
pub trait UserRepository {
    async fn create(&self, payload: CreateUserPayload) -> Result<User>;
    async fn find(&self, user_id: Uuid) -> Option<User>;
    async fn all(&self) -> Vec<User>;
    async fn update(&self, payload: UpdateUserPayload) -> Result<User>;
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait TodoRepository {
    async fn create(&self, payload: CreateTodoPayload) -> Result<Todo>;
    async fn find(&self, todo_id: Uuid) -> Option<Todo>;
    async fn all(&self) -> Vec<Todo>;
    async fn update(&self, payload: UpdateTodoPayload) -> Result<Todo>;
    async fn delete(&self, todo_id: Uuid) -> Result<()>;
}
