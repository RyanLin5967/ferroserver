use std::collections::HashMap;
use crate::error::ServerError;
use std::str::from_utf8;
use std::str::FromStr;

pub struct HttpRequest {
    method: Method,
    version: String,
    headers: HashMap<String, String>,
    path: String,
    query: Option<String>,
    body: Vec<u8>
}
pub enum Method {
    GET,
    POST,
    HEAD,
    DELETE,
}
impl FromStr for Method {
    type Err = ServerError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s{
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "HEAD" => Ok(Method::HEAD),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(ServerError::MethodNotAllowed),
        }
    }
}
pub fn parse(raw: &[u8]) -> Result<HttpRequest, ServerError>{
    let i = match raw.windows((b"\r\n\r\n").len()).position(|window| window == b"\r\n\r\n") {
        Some(ind) => ind,
        None => return Err(ServerError::Parse(String::from("didn't find the header cutoff")))
    };

    let raw_header = &raw[..i];
    let header_string = match from_utf8(raw_header){
        Ok(str) => str,
        Err(e) => return Err(ServerError::Parse(e.to_string()))
    };
    let mut lines = header_string.split("\r\n");
    let first = match lines.next(){
        Some(first_line) => first_line,
        None => return Err(ServerError::Parse(String::from("request is empty")))
    };
    let parts: Vec<&str> = first.split(" ").collect();
    let method: Method = parts[0].parse::<Method>()?;
    let uri: Vec<&str> = parts[1].split("?").collect();
    let path = String::from(uri[0]);
    let query = uri.get(1).map(|s| s.to_string());
    let version = String::from(parts[2]);

    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines {
        let parts:Vec<&str> = line.split(": ").collect();
        headers.insert(parts[0].to_lowercase(),parts[1].to_string());
    }
    let body_i = i+4;
    let body = match headers.get("content-length"){
        Some(len) => {
            raw[body_i..body_i +len.parse::<usize>().unwrap()].to_vec()
        }
        None => Vec::new()
    };
    return Ok(HttpRequest { method, version, headers, path, query, body })
}
    
