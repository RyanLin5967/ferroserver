use crate::{response::HttpResponse};
use flate2::{Compression, write::GzEncoder};
use std::io::Write;

pub fn compress(accepts_gzip: bool, response: &mut HttpResponse){

    if !is_compressible(response.headers.get("content-type").unwrap_or(&String::from(""))) || !accepts_gzip || response.body.len() < 150{
        return;
    }
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&response.body).unwrap();
    let new_body = encoder.finish().unwrap();
    let new_length = new_body.len();
    response.body = new_body;
    response.headers.insert(String::from("content-encoding"),String::from( "gzip"));
    response.headers.insert(String::from("content-length"), new_length.to_string());
    response.headers.insert(String::from("vary"), String::from("Accept-Encoding"));
}

fn is_compressible(content_type: &str) -> bool{
    return match content_type {
        "text/html" => true,
        "text/css" => true,
        "application/javascript" => true,
        "text/plain" => true,
        "image/svg+xml" => true,
        "image/png" => false,
        "image/jpeg" => false,
        "application/pdf" => false,
        "application/zip" => false,
        _ => false,
    }
}