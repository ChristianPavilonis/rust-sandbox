#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("LISTNING on {address}");

    axum::Server::bind(&address)
        .serve(routes.into_make_service())
        .await;
}

async fn main_response_mapper(response: Response) -> Response {
    println!("->> {:<12}", "RESPONSE_MAPPER");
    println!();

    response
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello2/:name", get(hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world");

    Html(format!("Hello, {name}!"))
}

async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {name}!"))
}
