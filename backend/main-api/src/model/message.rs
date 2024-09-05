use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use crate::{Error, Result};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgRow, query, FromRow, PgPool, Row};

// TYPES
#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub time_created: i64,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct MessageCreateInfo {
    pub title: String,
    pub content: String
}

#[derive(Deserialize)]
pub struct MessageFetchInfo {
    pub before_id: Option<u64>,
    pub amount: u8
}

// MODEL CONTROLLER
#[derive(Clone)]
pub struct MessageController {
    db_pool: PgPool
}

impl MessageController {
    pub async fn new(db_pool: PgPool) -> Result<Self> {
        Ok(
            Self {
                db_pool
            }
        )
    }
}

impl MessageController {
    pub async fn create_message(&self, create_info: MessageCreateInfo) -> Result<Message> {
        let start = SystemTime::now();
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

        let message = sqlx::query_as!(
            Message,
            "INSERT INTO messages_to_world (title, content, time_created)
            VALUES ($1, $2, $3)
            RETURNING *",
            create_info.title,
            create_info.content,
            since_the_epoch.as_secs() as i64
        ).fetch_one(&self.db_pool).await
        .map_err(|_| Error::DatabaseQueryError)?;

        Ok(message)
    }

    pub async fn list_messages(&self, fetch_info: MessageFetchInfo) -> Result<Vec<Message>> {
        if fetch_info.amount > 100 || fetch_info.amount == 0 {
            return Err(Error::BadValue { reason: String::from("Requested amount must be at least 1 and less than 100.") });
        }

        let messages = if let Some(before_id) = fetch_info.before_id {
            sqlx::query_as!(
                Message,
                "SELECT * FROM messages_to_world
                WHERE id < $1
                ORDER BY id DESC
                LIMIT $2",
                before_id as i64,
                fetch_info.amount as i64
            ).fetch_all(&self.db_pool).await
        } else {
            sqlx::query_as!(
                Message,
                "SELECT * FROM messages_to_world
                ORDER BY id DESC
                LIMIT $1",
                fetch_info.amount as i64
            ).fetch_all(&self.db_pool).await
        }.map_err(|_| Error::DatabaseQueryError)?;

        Ok(messages)
    }

    pub async fn delete_message(&self, id: u64) -> Result<Message> {
        
        let deleted_message = sqlx::query_as!(
            Message,
            "DELETE FROM messages_to_world
            WHERE id = $1
            RETURNING *",
            id as i64
        ).fetch_optional(&self.db_pool).await
        .map_err(|_| Error::DatabaseQueryError)?;

        deleted_message.ok_or(Error::MessageIdNotFound{id})
    }
}
