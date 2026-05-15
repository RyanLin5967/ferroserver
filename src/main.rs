mod server;
mod error;
mod request;
mod response;
mod static_files;
mod router;
mod handler;
mod connection;
mod compression;
use crate::handler::{echo, health};
use crate::request::Method;
use crate::router::Router;
use std::path::PathBuf;
use std::sync::Arc;

// ***NEED TO REFACTOR TO ASYNC FROM SYNC ***
#[tokio::main]
async fn main(){
    let path = PathBuf::from("./public");
    let mut router = Router::new(path);
    router.add(Method::GET, String::from("/api/health"), health);
    router.add(Method::POST, String::from("/api/echo"), echo);
    server::run("127.0.0.1:8000", Arc::new(router)).await;
}