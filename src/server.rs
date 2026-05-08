use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::path::Path;
use crate::request;
use crate::error::ServerError;
use crate::response::HttpResponse;
use crate::static_files::serve_file;

pub fn run(addr: &str){
    let listener =  match TcpListener::bind(&addr){
        Ok(listener) => {
            println!("bound to: {}", &addr);
            listener
        }
        Err(e) => {
            eprintln!("binding failed: {}", e);
            return;
        }
    };
    loop {
        match listener.accept(){
            Ok((stream, address)) => {
                println!("connected to: {}", address);
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("error: {}", e)
        };
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 4096];
    let size = match stream.read(&mut buffer){
        Ok(si) => si,
        Err(e) => {
            let _ = stream.write_all(&HttpResponse::from_error(&ServerError::Io(e.to_string())).to_bytes());
            return;
        }
    };
    let request = match request::parse(&buffer[..size]) {
        Ok(req) => req,
        Err(e) => {
            let _ = stream.write_all(&HttpResponse::from_error(&e).to_bytes());
            return;
        }
    };
    let response = match serve_file(Path::new("./public"), &request.path) {
        Ok(res) => res,
        Err(e) => HttpResponse::from_error(&e)
    };
    println!("{} {} {} {} bytes", request.method, request.path, response.status, response.body.len());
    let _ = stream.write_all(&response.to_bytes());
    let _ = stream.flush();
}
