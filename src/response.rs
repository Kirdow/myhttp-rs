use std::net::TcpStream;

use crate::headers::HttpHeaders;
use crate::request::HttpRequest;
use crate::http_error::{ HttpError, HttpCode };
use crate::io_util::{ write_line, write_body };
use crate::transcript::Transcript;

pub enum HttpResponseData {
    Content(String),
    Error(String),
    None
}

pub struct HttpResponse<'a> {
    pub request: HttpRequest,
    pub headers: HttpHeaders,
    stream: &'a TcpStream,
    pub error: Option<HttpError>,
    pub code: HttpCode,
    pub data: HttpResponseData
}

impl<'a> HttpResponse<'a> {
    pub fn new(request: HttpRequest, stream: &'a TcpStream) -> Self {
        Self {
            request,
            headers: HttpHeaders::new(),
            stream,
            error: None,
            code: HttpCode::E200,
            data: HttpResponseData::None
        }
    }

    fn get_error_content(code: HttpCode) -> String {
        format!("<html><body><h1>{}</h1></body></html>", code.get_desc())
    }

    pub fn set_error(&mut self, error: HttpError) {
        self.code = error.code.clone();
        self.error = Some(error.clone());
        self.data = HttpResponseData::Error(Self::get_error_content(error.code));
    }

    pub fn set_code(&mut self, code: HttpCode) {
        if code.is_error() {
            self.set_error(HttpError::new(code));
        } else {
            self.error = None;
            self.code = code;
            self.data = HttpResponseData::None;
        }
    }

    pub fn set_response(&mut self, code: HttpCode, content: String) {
        if code.is_error() {
            self.code = code.clone();
            self.error = Some(HttpError::new(code));
            self.data = HttpResponseData::Error(content);
        } else {
            self.error = None;
            self.code = code;
            self.data = HttpResponseData::Content(content);
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        let ts = &mut self.request.transcript;

        write_line(ts, self.stream, format!("HTTP/1.1 {}", self.code).as_str())?;
        if let Some(http_err) = &self.error {
            write_line(ts, self.stream, format!("X-Error-Info: {}", http_err.get_error_msg()).as_str())?;
            ts.push(format!("Full Error Info: {}", http_err).as_str());
        }

        for (key, value) in self.headers.iter() {
            if HttpHeaders::is_restricted_header(key) {
                continue;
            }

            write_line(ts, self.stream, format!("{}: {}", key, value).as_str())?;
        }

        match &self.data {
            HttpResponseData::Content(content) => {
                write_line(ts, self.stream, format!("Content-Type: {}", self.request.resource_type).as_str())?;
                write_body(ts, self.stream, content.as_str())?;
            },
            HttpResponseData::Error(content) => {
                write_line(ts, self.stream, "Content-Type: text/html")?;
                write_body(ts, self.stream, content.as_str())?;
            },
            HttpResponseData::None => {
                write_line(ts, self.stream, "Content-Type: text/html")?;
                write_body(ts, self.stream, Self::get_error_content(HttpCode::E501).as_str())?;
            }
        }

        Ok(())
    }
}