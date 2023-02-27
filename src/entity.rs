use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub user_id: Uuid,
    pub nickname: String,
    pub birthday: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CreateUserPayload {
    pub nickname: String,
    pub birthday: String,
}

#[derive(Serialize, Debug)]
pub struct Todo {
    pub todo_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub status: String,
    pub published_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CreateTodoPayload {
    pub user_id: Uuid,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UpdateTodoPayload {
    pub todo_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub status: String,
    pub edited_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(todo_id: Uuid, user_id: Uuid, title: String, now: DateTime<Utc>) -> Todo {
        Todo {
            todo_id,
            user_id,
            title,
            status: String::from("enable"),
            published_at: now,
            edited_at: now,
            created_at: now,
            updated_at: now,
        }
    }
}
