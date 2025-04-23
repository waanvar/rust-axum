#![allow(unused)]

pub use self::error::{Error, Result};

use crate::ctx::Ctx;
use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get , post , get_service};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()>{

    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // run our app with hyper, listening globally on port 3000
    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
 	println!("->> LISTENING on {:?}\n", listener.local_addr());
 	//axum::serve(listener, routes_all.into_make_service());

    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
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
        .route("/hello2/{name}",get(say_hello).post(handle_hello)) 
}

async fn say_hello() -> impl IntoResponse {
    Html("Hello World. i said.")
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
    age : Option<i32>,
    gender : Option<String>
}