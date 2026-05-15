use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::compression::compress;
use crate::request;
use crate::error::ServerError;
use crate::response::HttpResponse;
use crate::router::Router;
use std::sync::Arc;
use tokio::time::timeout;
use std::time::Duration;

// have to check connection header if close or keep alive
// if close, break, if not, no break
pub async fn handle_connection(mut stream: TcpStream, router: Arc<Router>){
    let mut close;
    let mut buf: Vec<u8> = Vec::new();
    let mut temp = [0u8; 4096];
    loop {
        loop {
            let _ = match buf.windows(4).position(|window| window == b"\r\n\r\n") {
                Some(_) => break,
                None => ()
            };
            let size = match timeout(Duration::from_secs(30), stream.read(&mut temp)).await{
                Ok(Ok(0)) => return, //client closed connection
                Ok(Ok(si)) => si,
                Ok(Err(e)) => {
                    let _ = stream.write_all(&HttpResponse::from_error(&ServerError::Io(e.to_string())).to_bytes()).await;
                    return;
                },
                Err(_) => return,
            };

            buf.extend_from_slice(&temp[..size]);
        }
        let (request, size) = match request::parse(&buf) {
            Ok(req) => req,
            Err(e) => {
                let _ = stream.write_all(&HttpResponse::from_error(&e).to_bytes()).await;
                return;
            }
        };
        buf.drain(..size);
        let connection = match &request.headers.get("connection") {
            Some(con) => con,
            None => "keep-alive"
        };
        if connection == "close" {
            close = true;
        }else {
            close = false;
        } 
        let accepts_gzip = request.headers.get("accept-encoding").map(|v| v.contains("gzip")).unwrap_or(false);
        let mut response = router.dispatch(request).await;
        compress(accepts_gzip, &mut response);
        let _ = stream.write_all(&response.to_bytes()).await;
        let _ = stream.flush().await;

        if close == true{
            break;
        }
    }
}