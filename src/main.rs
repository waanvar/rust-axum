#![allow(unused)]

pub use self::error::{Error,Result};

use axum::{
    extract::{Path, Query}, http::StatusCode, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service, post}, Router
};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routes_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    
    println!("-->> {:<12} - main_response_mapper", "HANDLER");

    println!("");
    res
}

fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router{
    Router::new()
        .route("/hello",get(say_hello).post(post_hello))
        .route("/hello2/{name}",post(handle_hello)) 
}

async fn say_hello() -> &'static str {
    "Hello World. i said."
}

async fn post_hello(Query(params):Query<User>) -> impl IntoResponse {
    //(StatusCode::OK,"Hello World. Post said.")
    println!("-->> {:<12} - say_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or(" Oh no!!!");

    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handle_hello(Path(name)  : Path<String>) -> impl IntoResponse {
    //(StatusCode::OK,"Hello World. Post said.")
    println!("-->> {:<12} - say_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

#[derive(Debug,Deserialize)]
struct User {
    name : Option<String>,
}