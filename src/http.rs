use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};
use serde_json;

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
        stream.write("end".as_bytes()).unwrap();
        let buf_reader = BufReader::new(stream);
        let mut hash_map: HashMap<String, String> = HashMap::new();
        let mut is_body_next = false;
        let is_body_read = false;
        let mut body_string = String::new();

        for (i, req_line) in buf_reader.lines().enumerate() {
            let line = req_line.unwrap();
            let key_value: Vec<&str> = line.split(": ").collect();
            match (
                key_value.len(),
                i,
                line.as_str(),
                is_body_next,
                is_body_read,
            ) {
                (1, 0, _, _, _) => {
                    hash_map.insert(String::from("method"), line);
                }
                (1, _, "", false, false) => is_body_next = true,
                (_, _, _, true, false) => {
                    if key_value.len() == 2 {
                        match key_value[1]
                            .replace("\"", "")
                            .replace(",", "")
                            .parse::<usize>()
                        {
                            Ok(_) => {
                                body_string.push_str(&key_value[0]);
                                body_string.push_str(": ");
                                body_string.push_str(&key_value[1].replace("\"", ""));
                                body_string.push_str("\n");
                            }
                            Err(_) => {
                                body_string.push_str(&line);
                            }
                        }
                    } else {
                        body_string.push_str(&line);
                    }
                }
                (_, _, _, _, _) => {
                    hash_map.insert(key_value[0].to_string(), key_value[1].to_string());
                }
            }
        }
        let headers = HttpHeader::new(&hash_map);
        let body: JsonBody = serde_json::from_str(&body_string).unwrap();
        Http { headers, body }
    }
}
