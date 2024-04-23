use std::net::TcpStream;

use crate::http_util::split_method;
use crate::http_error::{HttpError, http_errors};
use crate::io_util::get_stream_name;
use crate::headers::HttpHeaders;
use crate::transcript::Transcript;
use crate::util::get_time_str;

pub struct HttpRequest {
    pub who: String,
    pub transcript: Transcript,
    pub headers: HttpHeaders,
    pub path: String,
    pub resource_type: String,
    pub version: String,
    pub valid: bool,
    pub is_init: bool,
}

impl HttpRequest {
    pub fn new(stream: &TcpStream) -> Result<Self, HttpError> {
        Ok(Self {
            who: get_stream_name(stream),
            transcript: Transcript::new(stream)?,
            headers: HttpHeaders::new(),
            path: String::new(),
            resource_type: String::new(),
            version: String::new(),
            valid: false,
            is_init: false
        })
    }

    pub fn get_file_name(&self) -> String {
        if self.path.ends_with("/") {
            format!("{}index.html", self.path)
        } else {
            self.path.clone()
        }
    }

    fn init_resource_type(&mut self) -> Result<(), HttpError> {
        let resource_type;
        if self.path.ends_with(".html") {
            resource_type = "text/html";
        } else if self.path.ends_with(".png") {
            resource_type = "image/png";
        } else if self.path.ends_with(".ico") {
            resource_type = "image/x-icon";
        } else {
            return Err(http_errors::msg::forbidden("Invalid/Unaccepted resource type").set_info("Invalid MIME Type"));
        }

        self.resource_type = resource_type.to_string();

        Ok(())
    }

    pub fn init(&mut self, input: &String) -> Result<(), HttpError> {
        let (method, path, version) = split_method(input.as_str()).ok_or_else(|| http_errors::msg::bad_request("Request did not match <method> <path> <version> format").set_info("Malformed request"))?;

        if method == "GET" {
            self.transcript.push("GET Request")?;
            self.transcript.push(format!("Path: {}", path).as_str())?;
        } else {
            return Err(http_errors::msg::not_implemented(format!("Method {} is not implemented", method).as_str()).set_info("Unknown HTTP Method"));
        }

        self.path = if path.ends_with("/") {
            format!("{}index.html", path)
        } else {
            path.clone()
        };
        
        self.init_resource_type()?;

        if version != "HTTP/1.1" {
            return Err(http_errors::msg::not_implemented(format!("HTTP version {} is unsupported", version).as_str()).set_info("Unsupported HTTP version"));
        } else {
            self.version = version;
        }

        self.valid = true;
        self.is_init = true;

        Ok(())
    }

    pub fn feed(&mut self, input: &String) -> Result<(), HttpError> {
        if !self.is_init {
            return self.init(&input);
        }

        self.headers.add_from_line(input.as_str())?;

        Ok(())
    }
}
