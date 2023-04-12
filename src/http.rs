use pkg_version::*;
use std::{
    io::{self, Write},
    net::TcpStream,
};

const MAJOR: u32 = pkg_version_major!();
const MINOR: u32 = pkg_version_minor!();
const PATCH: u32 = pkg_version_patch!();

/// HTTP response builder.
pub(crate) struct HttpResponse {
    status: String,
    content: Option<String>,
    content_type: String,
}

impl HttpResponse {
    /// Create a new 200 OK response.
    pub fn ok() -> Self {
        HttpResponse {
            status: "HTTP/1.1 200 OK\r\n".to_owned(),
            content: None,
            content_type: "text/plain".to_owned(),
        }
    }

    /// Create a new 404 Not Found response.
    pub fn not_found() -> Self {
        HttpResponse {
            status: "HTTP/1.1 404 Not Found\r\n".to_owned(),
            content: Some("404 Not Found".to_owned()),
            content_type: "text/plain".to_owned(),
        }
    }

    /// Create a new 500 Internal Server Error response.
    pub fn err() -> Self {
        HttpResponse {
            status: "HTTP/1.1 500 Internal Server Error\r\n".to_owned(),
            content: Some("500 Internal Server Error".to_owned()),
            content_type: "text/plain".to_owned(),
        }
    }

    /// Add text content to the response.
    pub fn text(&mut self, content: &str) -> &Self {
        self.content = Some(content.to_owned());
        self
    }

    /// Add json content to the response.
    pub fn json(&mut self, content: &str) -> &Self {
        self.content = Some(content.to_owned());
        self.content_type = "application/json".to_owned();
        self
    }

    /// Send the HTTP response over Tcp.
    pub fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        let mut response = self.status.clone();

        if let Some(content) = &self.content {
            response.push_str(
                format!(
                    "Server: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                    format!("emberry@paperboy-{}.{}.{}", MAJOR, MINOR, PATCH),
                    content.len(),
                    self.content_type,
                    content
                )
                .as_str(),
            );
        } else {
            response.push_str("Content-Length: 0\r\n\r\n");
        }

        stream.write_all(response.as_bytes())
    }
}
