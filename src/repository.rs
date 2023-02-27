use anyhow::Result;
use thiserror::Error;

pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(String),
}

pub trait TodoRepository: Clone + std::marker::Send + std::maker::Sync + 'static {
    fn create(&self, payload: CreateTodoPayload) -> Result<Todo>;
    fn find(&self, id: Uuid) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, payload: UpdateTodoPayload) -> Result<Todo>;
    fn delete(&self, id: Uuid) -> Result<()>;
}
