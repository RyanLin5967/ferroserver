use crate::{error::ServerError, request::HttpRequest, response::HttpResponse};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use std::str::from_utf8;
use std::collections::HashMap;

pub async fn forward(mut request: HttpRequest, backend: &str) -> Result<HttpResponse, ServerError>{
    let mut stream = match TcpStream::connect(backend).await {
        Ok(s) => s,
        Err(_) => return Err(ServerError::Internal(String::from("502 Bad Gateway")))
    };
    request.headers.insert(String::from("host"), backend.to_string());
    let uri = match &request.query {
        Some(q) => format!("{}?{}", "/", q), //could also just strip the /proxy
        None => String::from("/")
    };
    let mut res = format!("{} {} HTTP/1.1\r\n", request.method, uri);
    request.headers.remove("content-length");
    for (key, value) in &request.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }
        res.push_str(&format!("Content-Length: {}\r\n", request.body.len()));
        res.push_str("\r\n");
        let mut bytes = res.as_bytes().to_vec();
        bytes.extend_from_slice(&request.body);
        
    let _ = stream.write_all(&bytes).await;
    let _ = stream.flush().await;

    let mut buf: Vec<u8> = Vec::new();
    let mut temp = [0u8; 4096];
    loop {
        let _ = match buf.windows(4).position(|window| window == b"\r\n\r\n") {
            Some(_) => break,
            None => ()
        };
        let size = stream.read(&mut temp).await?;
        buf.extend_from_slice(&temp[..size]);
    }
    let header_end = buf.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let header_str_temp = from_utf8(&buf[..header_end]).unwrap_or("");
    let content_length: usize = header_str_temp.split("\r\n")
        .filter_map(|line| line.split_once(": "))
        .find(|(k, _)| k.to_lowercase() == "content-length")
        .map(|(_, v)| v.parse().unwrap_or(0))
        .unwrap_or(0);

    let total_needed = header_end + 4 + content_length;
    while buf.len() < total_needed {
        let n = stream.read(&mut temp).await?;
        if n == 0 { break; }
        buf.extend_from_slice(&temp[..n]);
    }
    let i = match buf.windows(4).position(|win| win == b"\r\n\r\n") {
        Some(ind) => ind,
        None => return Err(ServerError::Parse(String::from("no header separater"))),
    };
    let raw_head = &buf[..i];
    let header_str = match from_utf8(raw_head) {
        Ok(str) => str,
        Err(e) => return Err(ServerError::Parse(e.to_string())),
    };
    let mut lines = header_str.split("\r\n");
    let first = match lines.next(){
        Some(fir) => fir,
        None => return Err(ServerError::Parse(String::from("empty request")))
    };

    let parts: Vec<&str> = first.split(" ").collect();
    // let version = String::from(parts[0]);
    let status = parts[1].parse::<u16>().unwrap();
    let reason = String::from(parts[2..].join(" "));

    let mut headers: HashMap<String, String> = HashMap::new();
    
    for line in lines {
        let partss: Vec<&str> = line.split(": ").collect();
        headers.insert(partss[0].to_lowercase(), partss[1].to_string());
    }
    let body_i = i+4;
    let body = match headers.get("content-length"){
        Some(len) => {
            if body_i + len.parse::<usize>().unwrap() > buf.len() {
                return Err(ServerError::Parse(String::from("incomplete body")))
            }
            buf[body_i..body_i + len.parse::<usize>().unwrap()].to_vec()
        }
        None => Vec::new()
    };
    headers.remove("content-length");
    return Ok(HttpResponse {status, reason, headers, body})

}