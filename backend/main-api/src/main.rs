mod error;
mod web;
mod models;
use error::{Error, Result};
use serde_json::json;
use crate::models::message::MessageModelController;

use axum::{extract::Query, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Json, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;
use dotenv::dotenv;
use std::env;

struct DbInfo {
    user: String,
    pass: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    dotenv().expect("Failed to load env from .env");
    
    // aquire database connectiosn etc etc
    
    let db_pool = PgPool::connect(
        env::var("DATABASE_URL").expect("Could not find DATABASE_URL in env").as_str()
    ).await.map_err(|_| Error::DatabaseConnectionError)?;

    sqlx::migrate!("../migrations").run(&db_pool).await.expect("Failed to run migrations.");

    let mc = MessageModelController::new(db_pool).await?;

    let message_routes = web::routes_messages::routes(mc.clone());

    let routes_all = Router::new()
    .nest("/api", message_routes)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    // start server
    let addr = "0.0.0.0:9099";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("-> LISTENING on {addr}\n");

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let error = res.extensions().get::<Error>();

    let sc_and_ce = error.map(|se| se.to_status_and_client_error());

    let error_response = sc_and_ce
        .as_ref()
        .map(|(status_code, client_err)| {
            let body = json!({
                "error": {
                    "type": client_err.as_ref()
                }
            });

            (*status_code, Json(body)).into_response()

        });

    error_response.unwrap_or(res)
}


// #[derive(Debug, Deserialize)]
// struct HelloParams {
//     name: Option<String>
// }

// async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
//     println!("->> {:<12} - handler_hello", "HANDLER");

//     let name = params.name.as_deref().unwrap_or("world");

//     Html(format!("Hello <strong>{name}!!!!!</strong>"))
// }