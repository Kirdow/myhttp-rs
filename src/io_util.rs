use std::{fs::File, io::{Read, Write}, net::TcpStream, path::PathBuf};

use crate::{http_error::HttpError, transcript::Transcript};

pub fn get_stream_name(stream: &TcpStream) -> String {
    stream.peer_addr().map(|addr| addr.to_string()).unwrap_or(String::from("Unknown Address"))
}

pub fn write_line(ts: &mut Transcript, mut stream: &TcpStream, line: &str) -> Result<(), HttpError> {
    ts.with_prefix("<--", |ts| ts.push(line))?;
    writeln!(stream, "{}\r", line).map_err(|e| HttpError::convert_from(e, Some("Failed to write line to HTTP stream")))
}

pub fn write_data(ts: &mut Transcript, mut stream: &TcpStream, data: &Vec<u8>) -> Result<(), HttpError> {
    ts.with_prefix("<--", |ts| ts.push("<binary data>"))?;
    stream.write(&data[..]).map_err(|e| HttpError::convert_from(e, Some("Failed to write binary data to HTTP stream")))?;

    Ok(())
}

pub fn write_body(ts: &mut Transcript, stream: &TcpStream, body: &str) -> Result<(), HttpError> {
    let len = body.len();

    write_line(ts, stream, format!("Content-Length: {}", len).as_str())?;
    write_line(ts, stream, "")?;
    write_line(ts, stream, body)
}

pub fn write_body_data(ts: &mut Transcript, stream: &mut TcpStream, data: &Vec<u8>) -> Result<(), HttpError> {
    let len = data.len();

    write_line(ts, stream, format!("Content-Length: {}", len).as_str())?;
    write_line(ts, stream, "")?;
    write_data(ts, stream, data)
}

pub fn read_binary_file(path: &str) -> Result<Vec<u8>, HttpError> {
    let mut file = File::open(path).map_err(|e| HttpError::convert_from(e, Some("Failed to open file")))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| HttpError::convert_from(e, Some("Failed to read file buffer")))?;

    Ok(buffer)
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

#[allow(unused)]
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

