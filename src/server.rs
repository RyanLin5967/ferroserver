use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::spawn;
use std::path::Path;
use crate::request;
use crate::error::ServerError;
use crate::response::HttpResponse;
use crate::router::Router;
use crate::static_files::serve_file;
use std::sync::Arc;


// needs to take in router as well
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

async fn handle_connection(mut stream: TcpStream, router: Arc<Router>){
    let mut buffer = [0; 4096];
    let size = match stream.read(&mut buffer).await{
        Ok(si) => si,
        Err(e) => {
            let _ = stream.write_all(&HttpResponse::from_error(&ServerError::Io(e.to_string())).to_bytes()).await;
            return;
        }
    };
    let request = match request::parse(&buffer[..size]) {
        Ok(req) => req,
        Err(e) => {
            let _ = stream.write_all(&HttpResponse::from_error(&e).to_bytes()).await;
            return;
        }
    };
    let response = router.dispatch(request);
    // println!("{} {} {} {} bytes", request.method, &request.path, response.status, response.body.len());
    let _ = stream.write_all(&response.await.to_bytes()).await;
    let _ = stream.flush().await;
}
