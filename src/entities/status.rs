use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum Status {
    Ok = 200,
    NotFound = 404,
    BadRequest = 400,
}

impl Status {
    pub fn get_message(&self) -> &str {
        match *self {
            Status::Ok => "Ok",
            Status::NotFound => "Not Found",
            Status::BadRequest => "Bad Request",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
