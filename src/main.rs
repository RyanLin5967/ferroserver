mod server;
mod error;
mod request;
mod response;
mod static_files;
mod router;
mod handler;
mod connection;
mod compression;
mod proxy;
use crate::handler::{echo, health};
use crate::request::Method;
use crate::router::Router;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main(){
    let path = PathBuf::from("./public");
    let mut router = Router::new(path);
    router.add(Method::GET, String::from("/api/health"), health);
    router.add(Method::POST, String::from("/api/echo"), echo);
    router.add_proxy(String::from("/proxy"), String::from("127.0.0.1:3001"));
    server::run("127.0.0.1:8000", Arc::new(router)).await;
}