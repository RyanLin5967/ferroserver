use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

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
    match stream.read(&mut buffer){
        Ok(size) => {
            println!("size: {}", size);
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, world");
            let _ = stream.flush();
            println!("wrote and flushed shit");
        }
        Err(_) => println!("error in reading data")
    }
}
