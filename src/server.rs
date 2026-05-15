use tokio::net::{TcpListener};
use tokio::spawn;
use crate::router::Router;
use std::sync::Arc;
use crate::connection::handle_connection;

pub async fn run(addr: &str, router: Arc<Router>){
    let listener =  match TcpListener::bind(&addr).await{
        Ok(listener) => {
            println!("bound to: {}", &addr);
            listener
        }
        Err(e) => {
            eprintln!("binding failed: {}", e);
            return;
        }
    };
    loop {
        match listener.accept().await{
            Ok((stream, address)) => {
                println!("connected to: {}", address);
                let clone = Arc::clone(&router);
                spawn(async move {
                    handle_connection(stream, clone).await;
                });
            }
            Err(e) => eprintln!("error: {}", e)
        };
    }
}


