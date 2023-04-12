mod http;
mod json;

use std::io::prelude::*;
use std::io;
use std::net::{TcpListener, TcpStream};
use http::HttpResponse;
use json::{reload_json, CACHED_JSON};

/// HTTP server host address.
const ADDR: &'static str = "127.0.0.1:1985";

fn main() {
    reload_json().expect("failed to load update json");

    let listener = TcpListener::bind(ADDR).unwrap();

    println!("Web server is listening at {}", ADDR);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            _ = handle_conn(stream);
        }
    }
}

/// Try parse the headers from a HTTP request.
fn parse_headers(buf: &[u8; 1024]) -> Option<String> {
    let Ok(content) = String::from_utf8(buf.to_vec()) else {
        return None;
    };

    let mut lines = content.lines();

    while let Some(line) = lines.next() {
        if line.starts_with("GET") {
            let mut split = line.split_whitespace();
            split.next();
            let Some(path) = split.next() else {
                return None;
            };
            return Some(path.to_owned());
        }
    }
    None
}

/// Fetch the content for a given API path.
fn fetch_path(path: &str) -> Option<String> {
    match path {
        "/updates" => {
            let json = CACHED_JSON.lock().expect("failed to lock json");
            Some(json.to_owned())
        }
        s if s.starts_with("/reload") => {
            if s.len() > 8 && &s[8..] == "1234" {
                reload_json().expect("failed to load update json");
                Some("New newspapers acquired".to_owned())
            } else {
                Some("Password required".to_owned())
            }
        }
        _ => None
    }
}

/// Handle an incoming connection.
fn handle_conn(mut stream: TcpStream) -> io::Result<()> {
    let path: Option<String>;
    {
        let mut buf = [0; 1024];
        stream.read(&mut buf)?;
        path = parse_headers(&buf);
    }

    if let Some(path) = path {
        if let Some(content) = fetch_path(&path) {
            HttpResponse::ok().text(&content).send(&mut stream)
        } else {
            HttpResponse::not_found().send(&mut stream)
        }
    } else {
        HttpResponse::err().send(&mut stream)
    }
}
