#![allow(unused)]

use crate::{web, Error, Result};
use serde::Deserialize;
use axum::{
    Json, 
    Router,
    routing::{post}
};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login",post(api_login))
}

async fn api_login(cookies: Cookies,payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login","HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
       return Err(Error::LoginFail);
    }

    //TODO : Set Cookies
    //FIXME : Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN,"user-1.exp.sign"));

    //Create the success body.
    let body = Json(json!({
        "result" : {
            "success" : true,
        }
    }));
    
    Ok(body)
}

#[derive(Debug,Deserialize)]
struct LoginPayLoad {
    username : String,
    pwd : String,
}