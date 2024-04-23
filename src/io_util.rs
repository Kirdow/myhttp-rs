use std::{fs::File, io::{Read, Write}, net::TcpStream, path::PathBuf};

use crate::{http_error::{http_errors, HttpError}, transcript::Transcript};

pub fn get_stream_name(stream: &TcpStream) -> String {
    stream.peer_addr().map(|addr| addr.to_string()).unwrap_or(String::from("Unknown Address"))
}

pub fn write_line(ts: &mut Transcript, mut stream: &TcpStream, line: &str) -> Result<(), HttpError> {
    ts.with_prefix("<--", |ts| ts.push(line))?;
    writeln!(stream, "{}\r", line).map_err(HttpError::from)
}

pub fn write_body(ts: &mut Transcript, stream: &TcpStream, body: &str) -> Result<(), HttpError> {
    let len = body.len();

    write_line(ts, stream, format!("Content-Length: {}", len).as_str())?;
    write_line(ts, stream, "")?;
    write_line(ts, stream, body)
}

pub fn read_all_file(path: &str) -> Result<String, HttpError> {
    let mut file = File::open(path).map_err(|err| {
        http_errors::msg::internal_server_error(format!("Failed to open file: {}", err.to_string()).as_str())
    })?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(HttpError::from)?;

    Ok(contents)
}

pub fn write_error(ts: &mut Transcript, stream: &TcpStream, http_err: HttpError) -> Result<(), HttpError> {
    write_line(ts, &stream, format!("HTTP/1.1 {}", http_err.code).as_str())?;
    write_line(ts, &stream, format!("X-Error-Info: {}", http_err).as_str())?;
    write_line(ts, &stream, "Connection: close")?;
    write_line(ts, &stream, "Connection-Type: text/html")?;

    let error_html = format!("<html><body><h1>{}</h1></body></html>", http_err.code.get_desc());
    write_body(ts, &stream, error_html.as_str())?;

    Ok(())
}

pub fn validate_path(path: &PathBuf, base_path: Option<&PathBuf>) -> Option<String> {
    println!("Path: {:?}", path);
    match path.canonicalize() {
        Ok(canonical_path) => {
            if let Some(base_path) = base_path {
                if (canonical_path.starts_with(base_path)) {
                    Some(canonical_path.display().to_string())
                } else {
                    None
                }
            } else {
                Some(canonical_path.display().to_string())
            }
        },
        Err(e) => {
            println!("Error validate path: {}", e);
            None
        }
    }
}

