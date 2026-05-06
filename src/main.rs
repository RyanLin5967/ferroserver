mod server;
mod error;
mod request;
mod response;
mod static_files;
fn main(){
    server::run("127.0.0.1:8000")
}