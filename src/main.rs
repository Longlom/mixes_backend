pub mod http;

use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

use crate::http::{APIKind, Http};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(100);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("stream - {:?}", stream);

        pool.execute(|| {
            handle_connection(stream);
        })
        // handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = Http::parse(&mut stream);
    match request.headers.method {
        APIKind::GetMain => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        APIKind::PostMain => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = "POST ACCEPTED";
            let contents_length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        _ => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = "SOMETHING WRONG BOYYYY";
            let contents_length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
   
}
