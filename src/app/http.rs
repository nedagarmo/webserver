use std::fmt::format;
use std::fs;
use crate::traits::Handler;
use crate::entities::{Request, Response, Status, Method};


pub struct WebHandler {
    root: String,
}

impl WebHandler {
    pub fn new(root: String) -> Self {
        WebHandler { root }
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let full_path = format!("{}/{}", self.root, file_path);

        match fs::canonicalize(full_path) {
            Ok(path) => {
                if path.starts_with(&self.root) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted to read: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebHandler{
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(Status::Ok, self.read_file("index.html")),
                path=> match self.read_file(&path) {
                    Some(content) => Response::new(Status::Ok, Some(content)),
                    None => Response::new(Status::NotFound, None),
                }
            },
            _ => Response::new(Status::NotFound, None),
        }
    }
}