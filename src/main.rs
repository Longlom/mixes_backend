pub mod http;

use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

use crate::http::Http;

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
    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    // for req_line in buf_reader.lines().enumerate() {
    //     println!("req_line - {:?}", req_line);
    // }
    // stream.set_nonblocking(true).expect("set_nonblocking call failed");
    // let mut buf = String::new();
    // loop {
    //     match stream.read_to_string(&mut buf) {
    //         Ok(_) => break,
    //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    //             // wait until network socket is ready, typically implemented
    //             // via platform-specific APIs such as epoll or IOCP
    //         }
    //         Err(e) => panic!("encountered IO error: {e}"),
    //     };
    // };
    // println!("bytes: {buf:?}");
    let http_data = Http::parse(&mut stream);
    println!("Body of data - {:?}", http_data)
    // let http_request_body: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("http_request_headers - {:?}", http_request_headers);

    // let mut buf_reader_iter = buf_reader.lines();
    // while let Some(data) = buf_reader_iter.next() {
    //     println!("req_line - {:?}", data);

    // }
    // println!("let request_line = buf_reader.lines() - {}", request_line);
    // println!("request_line - {}", request_line);
    // let (status_line, content_path) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    // let (status_line, api_kind) = match &request_line[..] {
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", APIKind::GetMain),
    //     "GET /sleep HTTP/1.1" => {
    //         thread::sleep(Duration::from_secs(5));
    //         ("HTTP/1.1 200 OK", APIKind::GetMain)
    //     },
    //     "POST / HTTP/1.1" => ("HTTP/1.1 200 OK", APIKind::PostMain),

    //     _ => ("HTTP/1.1 404 NOT FOUND", APIKind::Error),
    // };

    // // let contents = fs::read_to_string(content_path).unwrap();
    // let contents = match api_kind {
    //     APIKind::GetMain => fs::read_to_string("hello.html").unwrap(),
    //     APIKind::PostMain =>  {

    //         String::from("Saved")}
    //     APIKind::Error => fs::read_to_string("404.html").unwrap(),
    // };
    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();

    // // if request_line == "GET / HTTP/1.1" {
    // //     let status_line = "HTTP/1.1 200 OK";
    // //     let contents = fs::read_to_string("hello.html").unwrap();
    // //     let length = contents.len();

    // //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // //     stream.write_all(response.as_bytes()).unwrap();
    // // } else {
    // //     let status_line = "HTTP/1.1 404 NOT FOUND";
    // //     let contents = fs::read_to_string("404.html").unwrap();
    // //     let length = contents.len();

    // //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // // }
}
