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
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
}

#[async_trait]
pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateUserPayload) -> Result<User>;
    async fn find(&self, user_id: Uuid) -> Result<User>;
    async fn all(&self) -> Result<Vec<User>>;
    async fn update(&self, user_id: Uuid, payload: UpdateUserPayload) -> Result<User>;
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodoPayload) -> Result<Todo>;
    async fn find(&self, todo_id: Uuid) -> Result<Todo>;
    async fn all(&self) -> Result<Vec<Todo>>;
    async fn update(&self, todo_id: Uuid, payload: UpdateTodoPayload) -> Result<Todo>;
    async fn delete(&self, todo_id: Uuid) -> Result<()>;
}
