#![allow(unused)]

use axum::{
    extract::Query, 
    http::StatusCode, 
    response::{Html, IntoResponse}, 
    routing::get, Router
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router()).await.unwrap();
}

fn router() -> Router{
    Router::new().route("/hello",get(say_hello).post(post_hello))
}

async fn say_hello() -> &'static str {
    "Hello World. i said."
}

async fn post_hello(Query(params):Query<User>) -> impl IntoResponse {
    //(StatusCode::OK,"Hello World. Post said.")
    println!("-->> {:<12} - say_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or(" Oh no!!!");

    Html("Hello <strong>Hey</strong>")
}

#[derive(Debug,Deserialize)]
struct User {
    name : Option<String>,
}