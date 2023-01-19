use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str,
};

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug)]
pub enum APIKind {
    GetMain,
    PostMain,
    Error,
}
#[derive(Debug)]
pub struct HttpHeader {
    pub content_type: String,
    pub content_length: i32,
    pub method: APIKind,
}

impl HttpHeader {
    fn new(data: &HashMap<String, String>) -> HttpHeader {
        let content_type = match data.get("Content-Type") {
            Some(str) => str.clone(),
            None => String::from(""),
        };

        let content_length = match data.get("Content-Length") {
            Some(str) => str.parse::<i32>().unwrap(),
            None => 0,
        };
        let method = match &data.get("method").unwrap()[..] {
            "POST" => APIKind::PostMain,
            "GET" => APIKind::GetMain,
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
pub struct JsonFlavourEntity {
    flavour: String,
    percentage: u64,
    brand: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonMixEntity {
    name: String,
    components_count: usize,
    components: Vec<JsonFlavourEntity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBody {
    data: Vec<JsonMixEntity>,
}
#[derive(Debug)]
pub struct Http {
    pub headers: HttpHeader,
    pub body: JsonBody,
}

impl Http {
    pub fn parse(stream: &mut TcpStream) -> Http {
        let mut buf_reader = BufReader::new(stream);
        let mut method = String::from("");
        buf_reader.read_line(&mut method).unwrap();

        if method.starts_with("GET") {
            return Http::parse_get_request(buf_reader);
        } else {
            return Http::parse_post_request(buf_reader);
        }
    }

    fn parse_get_request(buffer: BufReader<&mut TcpStream>) -> Http {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        hash_map.insert(String::from("method"), String::from("GET"));
        for (_i, req_line) in buffer.lines().enumerate() {
            let line = req_line.unwrap();
            let key_value: Vec<&str> = line.split(": ").collect();
            match line.as_str() {
                "" => {
                    break;
                }
                _ => {
                    hash_map.insert(key_value[0].to_string(), key_value[1].to_string());
                }
            }
        }
        let headers = HttpHeader::new(&hash_map);
        let body = JsonBody { data: vec![] };
        Http { headers, body }
    }

    fn parse_post_request(mut buffer: BufReader<&mut TcpStream>) -> Http {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        hash_map.insert(String::from("method"), String::from("POST"));
        let mut line = String::from("");
        while let Ok(_) = buffer.read_line(&mut line) {
            let key_value: Vec<&str> = line.trim().split(": ").collect();
            if &line == "\r\n" {
                break;
            }
            hash_map.insert(key_value[0].to_string(), key_value[1].to_string());
            line = String::from("");
        }
        let body_size = hash_map
            .get("Content-Length")
            .unwrap()
            .parse::<usize>()
            .unwrap();
            
        let mut body_buffer = vec![0; body_size];
        buffer.read_exact(&mut body_buffer).unwrap();
        let body_string = str::from_utf8(&body_buffer).unwrap();
        let headers = HttpHeader::new(&hash_map);
        let body = serde_json::from_str::<JsonBody>(&body_string).unwrap();
        Http { headers, body }
    }
}
