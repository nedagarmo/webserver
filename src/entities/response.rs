use std::fmt::Display;
use std::io::{Write, Result as IoResult};
use super::Status;

#[derive(Debug)]
pub struct Response {
    status: Status,
    body: Option<String>,
}

impl Response {
    pub fn new(status: Status, body: Option<String>) -> Self {
        Response {status, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };

        write!(stream, "HTTP/1.1 {} {}\n\r\n\r{}", self.status, self.status.get_message(), body)
    }
}