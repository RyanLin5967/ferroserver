use crate::request::Method;
use std::collections::HashMap;

pub struct HttpResponse {
    status: u16,
    reason: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl HttpResponse {
    pub fn ok(body: Vec<u8>, content_type: &str) -> HttpResponse{
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type.to_string());
        HttpResponse {
            status: 200,
            reason: "OK".to_string(),
            headers,
            body
        }
    }

    pub fn not_found() -> HttpResponse{
        let mut headers = HashMap::new();
        headers.insert(String::from("Content-Type"), String::from("text/html"));
        HttpResponse {
            status: 404,
            reason: String::from("Not Found"),
            headers,
            body: b"<h1>404 Not Found </h1>".to_vec()
        }
    }

    pub fn bad_request(reason: &str) -> HttpResponse{
        let mut headers = HashMap::new();
        headers.insert(String::from("Content-Type"), String::from("text/html"));
        HttpResponse {
            status: 400,
            reason: String::from("Bad Request"),
            headers,
            body: format!("<h1>400 Bad Request: {}</h1>", reason).into_bytes()
        }
    }

    pub fn method_not_allowed(allowed: &[Method]) -> HttpResponse{
        let mut headers = HashMap::new();
        headers.insert(String::from("Content-Type"), String::from("text/html"));
        let allow_value = allowed.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(", ");
        headers.insert(String::from("Allow"), allow_value);
        HttpResponse {
            status: 405,
            reason: String::from("Method Not Allowed"),
            headers,
            body: b"<h1>405 Method Not Allowed </h1>".to_vec()
        }
    }

    pub fn internal_error() -> HttpResponse{
        let mut headers = HashMap::new();
        headers.insert(String::from("Content-Type"), String::from("text/html"));
        HttpResponse {
            status: 500,
            reason: String::from("something went wrong..."),
            headers,
            body: b"<h1>500 Internal Error </h1>".to_vec(),
        }
    }
}