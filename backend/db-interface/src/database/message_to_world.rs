use crate::{statics::{DB_POOL}};

use anyhow::Result;

use sqlx::{
    Row,
};

pub struct MessageToWorld {
    pub title: String,
    pub content: String,
    pub time_created: i64
}

pub async fn save(msg: &MessageToWorld) -> Result<()> {
    let query = "INSERT INTO messages_to_world (title, content, time_created) VALUES ($1, $2, $3)";

    let guard = DB_POOL.lock().await;
    let pool = guard.as_ref().unwrap();

    sqlx::query(query)
        .bind(&msg.title)
        .bind(&msg.content)
        .bind(&msg.time_created)
        .execute(pool)
        .await?;

    Ok(())

}