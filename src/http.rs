use std::{io::{self, Write}, net::TcpStream};

/// HTTP response builder.
pub(crate) struct HttpResponse {
    status: String,
    content: Option<String>
}

impl HttpResponse {
    /// Create a new 200 OK response.
    pub fn ok() -> Self {
        HttpResponse { status: "HTTP/1.1 200 OK\r\n".to_owned(), content: None }
    }

    /// Create a new 404 Not Found response.
    pub fn not_found() -> Self {
        HttpResponse { status: "HTTP/1.1 404 Not Found\r\n".to_owned(), content: None }
    }

    /// Create a new 500 Internal Server Error response.
    pub fn err() -> Self {
        HttpResponse { status: "HTTP/1.1 500 Internal Server Error\r\n".to_owned(), content: None }
    }

    /// Add a text content to the response.
    pub fn text(&mut self, content: &str) -> &Self {
        self.content = Some(content.to_owned());
        self
    }

    /// Send the HTTP response over Tcp.
    pub fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        let mut response = self.status.clone();

        if let Some(content) = &self.content {
            response.push_str(format!("Content-Length: {}\r\n\r\n{}", content.len(), content).as_str());
        } else {
            response.push_str("Content-Length: 0\r\n\r\n");
        }

        stream.write_all(response.as_bytes())
    }
}