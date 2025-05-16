use crate::entities::{ParseError, Request, Response};
use crate::entities::Status;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        dbg!(e);
        println!("Failed to parse request: {:?}", e);
        Response::new(Status::BadRequest, None)
    }
}