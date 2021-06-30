use super::method::MethodError;
use super::method::Method;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;
use super::{QueryString, Headers};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    body: Option<&'buf str>,
    query_string: Option<QueryString<'buf>>,
    method: Method,
    headers: Option<Headers<'buf>>,

}

impl<'buf> TryFrom<&'buf[u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf:&'buf[u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;
        
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        let raw_headers = request.trim_start();
        let mut headers = None;
        let mut body = None;
        if let Some(i) = raw_headers.find("\r\n\r\n") {
            headers = Some(Headers::from(&raw_headers[..i]));
            if let Some(j) = raw_headers[i+4..].find("\0") {
                body = match j + 4 {
                    i => None,
                    _ => Some(&raw_headers[i+4..j]),
                }
            }
        }

        Ok(
            Self {
                path,
                body,
                query_string,
                headers,
                method,
            }
        )
    }
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
         &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self { 
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self { 
        Self::InvalidMethod
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c =='\r' {
            return Some((&request[..i], &request[i+1..]))
        }
    }

    None
}