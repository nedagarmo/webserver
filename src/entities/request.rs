use super::method::{ Method, MethodError};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};
use super::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<QueryString<'buffer>>,
    method: Method,
}

impl<'buffer> Request<'buffer> {
    fn get_next_value(request: &str) -> Option<(&str, &str)> {
        for (i, c) in request.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some((&request[..i], &request[i + 1..]));
            }
        }

        None
    }
}

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    fn try_from(buffer: &'buffer [u8]) -> Result<Request<'buffer>, Self::Error> {
        let request = from_utf8(buffer)?;
        let (method, request) = Self::get_next_value(request).ok_or(ParseError::InvalidRequest("The request is not following a correct format.".to_string()))?;
        let (mut path, request) = Self::get_next_value(request).ok_or(ParseError::InvalidRequest("The request is not following a correct format.".to_string()))?;
        let (protocol, _) = Self::get_next_value(request).ok_or(ParseError::InvalidRequest("The request is not following a correct format.".to_string()))?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol("The request protocol is not HTTP/1.1.".to_string()));
        }

        let method: Method = method.parse()?;
        let mut query_string: Option<QueryString<'buffer>> = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1 ..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

impl Display for Request<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.path)
    }
}


pub enum ParseError {
    InvalidRequest(String),
    InvalidEncoding(String),
    InvalidProtocol(String),
    InvalidMethod(String),
}

impl ParseError {
    fn message(&self) -> String {
        match self {
            ParseError::InvalidRequest(message ) => format!("Invalid Request: {}", message),
            ParseError::InvalidEncoding(message) => format!("Invalid Encoding: {}", message),
            ParseError::InvalidProtocol(message) => format!("Invalid Protocol: {}", message),
            ParseError::InvalidMethod(message) => format!("Invalid Method: {}", message),
        }
    }
}

impl Error for ParseError {
}

impl From<MethodError> for ParseError {
    fn from(error: MethodError) -> Self {
        Self::InvalidMethod(error.into_inner())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding("Request has invalid characters".to_string())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}