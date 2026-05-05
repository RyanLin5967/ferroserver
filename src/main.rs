mod server;
mod error;
mod request;
fn main(){
    server::run("127.0.0.1:8000")
}