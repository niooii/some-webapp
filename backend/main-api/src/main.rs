mod error;
mod ctx;
mod web;
mod model;
use error::{Error, Result};

use axum::{Router, routing::{get, get_service}, response::{Html, IntoResponse, Response}, extract::Query, middleware};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

use crate::model::ModelController;

struct DbInfo {
    user: String,
    pass: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // init the model controller
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_messages::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // start server
    let addr = "127.0.0.1:9099";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("-> LISTENING on {addr}\n");

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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