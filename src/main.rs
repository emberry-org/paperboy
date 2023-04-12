mod http;
mod json;

use chrono::{Utc, DateTime};
use http::HttpResponse;
use json::{reload_json, CACHED_JSON};
use pkg_version::*;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const MAJOR: u32 = pkg_version_major!();
const MINOR: u32 = pkg_version_minor!();
const PATCH: u32 = pkg_version_patch!();

/// HTTP server host address.
const ADDR: &'static str = "127.0.0.1:1985";

fn main() {
    // Initial load of the update json.
    reload_json().expect("failed to load update json");

    let listener = TcpListener::bind(ADDR).unwrap();

    println!("\r\n~ Paperboy v{}.{}.{}", MAJOR, MINOR, PATCH);
    println!("Waiting for requests at {}", ADDR);

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
        return match line {
            s if s.starts_with("GET") => {
                let mut split = line.split_whitespace();
                split.next();
                let Some(path) = split.next() else {
                    return None;
                };
                Some(path.to_owned())
            }
            s if s.starts_with("PATCH") => {
                let mut split = line.split_whitespace();
                split.next();
                let Some(path) = split.next() else {
                    return None;
                };
                let mut path = path.to_owned();
                path.insert_str(0, "patch");
                Some(path)
            }
            _ => None,
        };
    }
    None
}

/// Fetch the content for a given API path.
fn fetch_path(path: &str, http: &mut HttpResponse) -> bool {
    match path {
        "/updates" => {
            let json = CACHED_JSON.lock().expect("failed to lock json");
            http.json(&json);
            true
        }
        s if s.starts_with("patch/reload") => {
            if s.len() > 13 && &s[13..] == "1234" {
                let new_version = reload_json().expect("failed to load update json");
                let now: DateTime<Utc> = Utc::now();
                println!(" > Reloaded newspapers, latest version: {}   '{}'", new_version, now.format("%d/%m/%Y %H:%M"));
                http.text("New newspapers acquired");
            } else {
                http.text("Password required");
            }
            true
        }
        _ => false,
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
        let mut response = HttpResponse::ok();

        if fetch_path(&path, &mut response) {
            response.send(&mut stream)
        } else {
            HttpResponse::not_found().send(&mut stream)
        }
    } else {
        HttpResponse::err().send(&mut stream)
    }
}
