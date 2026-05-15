use std::collections::HashMap;
use crate::error::ServerError;
use std::str::from_utf8;
use std::str::FromStr;

#[derive(Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub path: String,
    pub query: Option<String>,
    pub body: Vec<u8>
}
#[derive(PartialEq)]
#[derive(Clone)]
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

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::HEAD => write!(f, "HEAD"),
            Method::DELETE => write!(f, "DELETE")
        }
    }
}

// ADD ACTUAL EDGE CASE DETECTION LATER (no unwraps)
pub fn parse(raw: &[u8]) -> Result<(HttpRequest, usize), ServerError>{
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
            if body_i + len.parse::<usize>().unwrap() > raw.len() {
                return Err(ServerError::Parse(String::from("incomplete body")))
            }
            raw[body_i..body_i +len.parse::<usize>().unwrap()].to_vec()
        }
        None => Vec::new()
    };
    let consumed = body_i + body.len();
    return Ok((HttpRequest { method, version, headers, path, query, body }, consumed))
}
    
