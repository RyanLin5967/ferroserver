use crate::response::HttpResponse;
use crate::error::ServerError;
use std::path::Path;
use tokio::fs::read;

pub async fn serve_file(root: &Path, request_path: &str) -> Result<HttpResponse, ServerError>{
    if request_path.contains(".."){
        return Err(ServerError::BadRequest(String::from("nice try buddy")));
    }
    if request_path.contains('\0'){
        return Err(ServerError::BadRequest(String::from("path contains a null byte")));
    }
    let full_path = root.join(request_path.trim_start_matches("/"));
    let file = read(&full_path).await?;

    let mime_type = match full_path.extension().unwrap().to_str() {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("gif") => "image/gif",
        Some("ico") => "image/x-icon",
        Some("txt") => "text/plain",
        Some("pdf") => "application/pdf",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream"
    };
    return Ok(HttpResponse::ok(file, mime_type));
}