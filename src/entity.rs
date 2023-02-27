use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
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
    pub todo_id_: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub status: String,
    pub published_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
