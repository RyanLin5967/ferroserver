use std::path::PathBuf;

use crate::{error::ServerError, request::{HttpRequest, Method}, response::HttpResponse, static_files::serve_file};

pub struct Route {
    method: Method,
    pattern: String,
    handler: fn(HttpRequest) -> Result<HttpResponse, ServerError>,
}

pub struct Router {
    routes: Vec<Route>,
    root_path: PathBuf
}

impl Router {
    pub fn add(&mut self, method: Method, pattern: String, handler: fn(HttpRequest) -> Result<HttpResponse, ServerError>){
        self.routes.push(Route{method, pattern, handler})
    }

    pub fn new(static_root: PathBuf) -> Router {
        return Router { routes: Vec::new(), root_path: static_root }
    }

    pub fn dispatch(&self, request: HttpRequest) -> HttpResponse{
        for route in &self.routes {
            if route.method == request.method && route.pattern == request.path {
                return match (route.handler)(request) {
                    Ok(res) => res,
                    Err(e) => HttpResponse::from_error(&e)
                }
            }
        }
        return match serve_file(&self.root_path, &request.path) {
            Ok(res) => res,
            Err(e) => HttpResponse::from_error(&e)
        };
    }
}

