use crate::traits::Handler;
use crate::entities::{Request, Response, Status};


pub struct WebHandler;

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(Status::Ok, Some("<h1>Hello</h1>".to_string()))
    }
}