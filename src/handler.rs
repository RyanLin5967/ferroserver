use crate::{error::ServerError, request::{HttpRequest, Method}, response::HttpResponse};

// for testing get
pub fn health(request: HttpRequest) -> Result<HttpResponse, ServerError>{
    let body = b"{\"status\": \"ok\"}";
    return Ok(HttpResponse::ok( body.to_vec(), "application/json"));
}

// for testing post
pub fn echo(request: HttpRequest) -> Result<HttpResponse, ServerError>{
     return Ok(HttpResponse::ok(request.body, request.headers.get("content-type").unwrap()));
}