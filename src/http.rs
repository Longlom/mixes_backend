use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug)]
pub enum APIKind {
    GetMain,
    PostMain,
    Error,
}
#[derive(Debug)]
pub struct HttpHeader {
    content_type: String,
    content_length: i32,
    method: APIKind,
}

impl HttpHeader {
    fn new(data: &HashMap<String, String>) -> HttpHeader {
        let content_type = data.get("Content-Type").unwrap().clone();
        let content_length = data
            .get("Content-Length")
            .unwrap()
            .parse::<i32>()
            .unwrap_or_default();
        let method = match &data.get("method").unwrap()[..] {
            "POST / HTTP/1.1" => APIKind::PostMain,
            "GET / HTTP/1.1" => APIKind::GetMain,
            _ => APIKind::Error,
        };

        HttpHeader {
            content_type,
            content_length,
            method,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBody {
    data: String,
}
#[derive(Debug)]
pub struct Http {
   pub headers: HttpHeader,
   pub body: JsonBody,
}

impl Http {
    pub fn parse(stream: &mut TcpStream) -> Http {
        stream.flush().unwrap();
        let mut buf_reader = BufReader::new(stream);
        let mut hash_map: HashMap<String, String> = HashMap::new();
        let mut is_next_body = false;
        for (i, req_line) in buf_reader.lines().enumerate() {
            if i == 0 {
                hash_map.insert(String::from("method"), req_line.unwrap());
            } else if req_line.as_ref().unwrap() == "" {
                is_next_body = true;
                println!("")
                // break;
            } else if is_next_body {
                hash_map.insert(String::from("data"), req_line.unwrap());
                break;
            } else {
                let line = req_line.unwrap();
                let key_value: Vec<&str> = line.split(": ").collect();
                hash_map.insert(
                    key_value[0].clone().to_string(),
                    key_value[1].clone().to_string(),
                );
            }
        }

        // println!("hash_map - {:?} ", hash_map);
        let headers = HttpHeader::new(&hash_map);
        let body: JsonBody = serde_json::from_str(hash_map.get("data").unwrap()).unwrap();

        Http {
            headers,
            body
        }
        // println!(
        //     "hash_map.get.unwrap().parse::<u64>().unwrap() - {:?} ",
        //     hash_map
        //         .get("Content-Length")
        //         .unwrap()
        //         .parse::<u64>()
        //         .unwrap()
        // );
        // let body: JsonValue;
        // let mut buf_reader_iter = buf_reader.lines();
        // // while let Some(http_line) = buf_reader_iter.next() {
        // //     if http_line.unwrap().as_str().contains("")
        // // }
    }
}
