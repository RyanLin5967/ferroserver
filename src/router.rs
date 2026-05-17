use std::path::PathBuf;

use crate::{error::ServerError, proxy::forward, request::{HttpRequest, Method}, response::HttpResponse, static_files::serve_file};

pub struct Route {
    method: Method,
    pattern: String,
    handler: fn(HttpRequest) -> Result<HttpResponse, ServerError>,
}

pub struct Router {
    routes: Vec<Route>,
    proxy_routes: Vec<(String, String)>,
    root_path: PathBuf
}

impl Router {
    pub fn add(&mut self, method: Method, pattern: String, handler: fn(HttpRequest) -> Result<HttpResponse, ServerError>){
        self.routes.push(Route{method, pattern, handler})
    }

    pub fn add_proxy(&mut self, pattern: String, backend: String){
        self.proxy_routes.push((pattern, backend));
    }

    pub fn new(static_root: PathBuf) -> Router {
        return Router { routes: Vec::new(), root_path: static_root, proxy_routes: Vec::new() }
    }

    pub async fn dispatch(&self, request: HttpRequest) -> HttpResponse{
        for (pattern, backend) in &self.proxy_routes {
            if request.path.starts_with(pattern) {
                let _res = match forward(request, backend).await {
                    Ok(resp) => return resp,
                    Err(e) => return HttpResponse::from_error(&e)
                };
            }
        }
        for route in &self.routes {
            if route.method == request.method && route.pattern == request.path {
                return match (route.handler)(request) {
                    Ok(res) => res,
                    Err(e) => HttpResponse::from_error(&e)
                }
            }
        }
        return match serve_file(&self.root_path, &request.path).await {
            Ok(res) => res,
            Err(e) => HttpResponse::from_error(&e)
        };
    }
}

