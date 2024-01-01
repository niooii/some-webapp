pub mod database;
mod statics;

use statics::DB_POOL;
use sqlx::postgres::PgPool;

use anyhow::Result;

const DB_URL: &str = "postgres://niooi:abcde@localhost:9432/postgres";

/// This must be called before calling any other function.
pub async fn init_pool() -> Result<()> {
    let mut guard = DB_POOL.lock().await;

    let pool = PgPool::connect(DB_URL).await?;

    sqlx::migrate!("../migrations").run(&pool).await?;

    let _ = guard.insert(pool);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn try_postgres_connect() {
        init_pool().await.expect("Could not start postgres connection.");
    }
}
