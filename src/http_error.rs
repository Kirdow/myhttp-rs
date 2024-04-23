
use std::io::{Error, ErrorKind};

use crate::str_util::Builder;

#[derive(Clone)]
pub enum HttpCode {
    E200,
    E400,
    E403,
    E404,
    E500,
    E501
}

pub enum HttpCodeRange {
    Info,
    Success,
    Redirection,
    ClientError,
    ServerError
}

#[derive(Clone)]
pub struct HttpError {
    pub code: HttpCode,
    pub desc: Option<String>,
    pub info: Option<String>
}

#[allow(unused)]
impl HttpError {
    pub fn new(code: HttpCode) -> Self {
        Self {
            code: code,
            desc: None,
            info: None
        }
    }

    pub fn new_with_message(code: HttpCode, msg: &str) -> Self {
        Self {
            code: code,
            desc: Some(msg.to_string()),
            info: None
        }
    }

    pub fn from_code(code: i32) -> Self {
        Self::new(HttpCode::from(code))
    }

    pub fn get_error_msg(&self) -> String {
        if let Some(info) = &self.info {
            format!("ERROR {}: {}", self.code, info)
        } else {
            format!("ERROR {}", self.code)
        }
    }

    pub fn set_info(&mut self, msg: &str) -> HttpError {
        self.info = Some(msg.to_string());
        self.to_owned()
    }

    pub fn convert_from(e: Error, msg: Option<&str>) -> Self {
        if let Some(msg) = msg {
            println!("HTTP <-- IO Error: \"{}\" Reason: {}", e.to_string(), msg);
            http_errors::msg::internal_server_error(format!("HTTP Error: \"{}\" from IO Error: \"{}\"", msg, e.to_string()).as_str())
        } else {
            println!("HTTP <-- IO Error: {}", e.to_string());
            http_errors::msg::internal_server_error(format!("HTTP Error: {}", e.to_string()).as_str())
        }
    }

    pub fn convert_to(&self, msg: Option<&str>) -> Error {
        if let Some(msg) = msg {
            println!("IO <-- HTTP Error: \"{}\" Reason: {}", self.to_string(), msg);
            Error::new(ErrorKind::Other, format!("IO Error: \"{}\" from HTTP Error: \"{}\"", msg, self.to_string()).as_str())
        } else {
            println!("IO <-- HTTP Error: {}", self.to_string());
            Error::new(ErrorKind::Other, format!("IO Error: {}", self.to_string()))
        }
    }

    pub fn convert_to_direct(err: HttpError) -> Error {
        err.convert_to(None)
    }
}

// Normally wouldn't allow unused, but these are general helpers across the board,
// and some might not be used now, but they might be used in the future
#[allow(unused)]
pub mod http_errors {
    pub mod msg {
        use super::super::{ HttpError, HttpCode};

        pub fn bad_request(msg: &str) -> HttpError {
            HttpError::new_with_message(HttpCode::E400, msg)
        }

        pub fn forbidden(msg: &str) -> HttpError {
            HttpError::new_with_message(HttpCode::E403, msg)
        }

        pub fn not_found(msg: &str) -> HttpError {
            HttpError::new_with_message(HttpCode::E404, msg)
        }

        pub fn internal_server_error(msg: &str) -> HttpError {
            HttpError::new_with_message(HttpCode::E500, msg)
        }

        pub fn not_implemented(msg: &str) -> HttpError {
            HttpError::new_with_message(HttpCode::E501, msg)
        }
    }

    use super::{ HttpError, HttpCode };

    pub fn bad_request() -> HttpError {
        HttpError::new(HttpCode::E400)
    }

    pub fn forbidden() -> HttpError {
        HttpError::new(HttpCode::E403)
    }

    pub fn not_found() -> HttpError {
        HttpError::new(HttpCode::E404)
    }

    pub fn internal_server_error() -> HttpError {
        HttpError::new(HttpCode::E500)
    }

    pub fn not_implemented() -> HttpError {
        HttpError::new(HttpCode::E501)
    }
}

#[allow(unused)]
impl HttpCode {
    pub fn from(code: i32) -> Self {
        match code {
            200 => HttpCode::E200,
            400 => HttpCode::E400,
            403 => HttpCode::E403,
            404 => HttpCode::E404,
            500 => HttpCode::E500,
            501 => HttpCode::E501,
            _ => HttpCode::E501
        }
    }

    pub fn get_code(&self) -> i32 {
        match self {
            HttpCode::E200 => 200,
            HttpCode::E400 => 400,
            HttpCode::E403 => 403,
            HttpCode::E404 => 404,
            HttpCode::E500 => 500,
            HttpCode::E501 => 501,
        }
    }

    pub fn get_desc(&self) -> &str {
        match self {
            HttpCode::E200 => "OK",
            HttpCode::E400 => "Bad Request",
            HttpCode::E403 => "Forbidden",
            HttpCode::E404 => "Not Found",
            HttpCode::E500 => "Internal Server Error",
            HttpCode::E501 => "Not Implemented",
        }
    }

    pub fn get_type(&self) -> HttpCodeRange {
        let number = self.get_code();
        if number >= 500 {
            assert!(number <= 599, "Invalid HTTP Response code. Need to be below 599");
            HttpCodeRange::ServerError
        } else if number >= 400 {
            HttpCodeRange::ClientError
        } else if number >= 300 {
            HttpCodeRange::Redirection
        } else if number >= 200 {
            HttpCodeRange::Success
        } else {
            assert!(number >= 100, "Invalid HTTP Response code. Need to be at least 100");
            HttpCodeRange::Info
        }
    }

    pub fn is_error(&self) -> bool {
        let code_type = self.get_type();
        matches!(code_type, HttpCodeRange::ClientError) || matches!(code_type, HttpCodeRange::ServerError)
    }
}

impl std::fmt::Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.get_code(), self.get_desc())
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut msg = Builder::new(&String::from(" | "));

        msg.try_append(&self.info);
        msg.try_append(&self.desc);
        
        match msg.get(true) {
            Some(msg) => {
                write!(f, "ERROR {}: {}", self.code, msg)
            },
            None => {
                write!(f, "ERROR {}", self.code)
            }
        }
    }
}