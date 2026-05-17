use crate::{error::ServerError, request::{HttpRequest}, response::HttpResponse};

// for testing get
pub fn health(_request: HttpRequest) -> Result<HttpResponse, ServerError>{
    let body = b"{\"status\": \"ok\"}";
    return Ok(HttpResponse::ok( body.to_vec(), "application/json"));
}

// for testing post
pub fn echo(request: HttpRequest) -> Result<HttpResponse, ServerError>{
    let content_type = match request.headers.get("content-type"){
        Some(ct) => ct,
        None => "text/plain"
    };
    return Ok(HttpResponse::ok(request.body, content_type));
}