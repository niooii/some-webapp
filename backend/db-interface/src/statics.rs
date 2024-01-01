use lazy_static::lazy_static;
use tokio::sync::Mutex;
use sqlx::postgres::PgPool;




lazy_static! {
    pub static ref DB_POOL: Mutex<Option<PgPool>> = Mutex::new(None);
}