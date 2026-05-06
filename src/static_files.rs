use crate::response::HttpResponse;
use crate::error::ServerError;
use std::path::Path;
use std::collections::HashMap;
use std::fs::read;

pub fn serve_file(root: &Path, request_path: &str) -> Result<HttpResponse, ServerError>{
    if request_path.contains(".."){
        return Err(ServerError::BadRequest(String::from("nice try buddy")));
    }
    if request_path.contains('\0'){
        return Err(ServerError::BadRequest(String::from("path contains a null byte")));
    }
    let full_path = root.join(request_path);
    let file = read(full_path)?;
    let ext = root.extension().unwrap();

    let mime_type = match ext.to_str() {
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