use std::{fs::File, io::Read, io::Write, net::TcpStream};

use crate::http_error::{HttpError, http_errors};

pub fn get_stream_name(stream: &TcpStream) -> String {
    stream.peer_addr().map(|addr| addr.to_string()).unwrap_or(String::from("Unknown Address"))
}

pub fn write_line(mut stream: &TcpStream, line: &str) -> std::io::Result<()> {
    println!("{} <-- {}", stream.peer_addr()?, line);
    writeln!(stream, "{}\r", line)
}

pub fn write_body(stream: &TcpStream, body: &str) -> std::io::Result<()> {
    let len = body.len();

    write_line(stream, format!("Content-Length: {}", len).as_str())?;
    write_line(stream, "")?;
    write_line(stream, body)
}

pub fn read_all_file(path: &str) -> Result<String, HttpError> {
    let mut file = File::open(path).map_err(|err| {
        http_errors::msg::internal_server_error(format!("Failed to open file: {}", err.to_string()).as_str())
    })?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| {
        http_errors::msg::internal_server_error(format!("Failed to read file contents: {}", err.to_string()).as_str())
    })?;

    Ok(contents)
}

pub fn write_error(stream: &TcpStream, http_err: HttpError) -> std::io::Result<()> {
    write_line(&stream, format!("HTTP/1.1 {}", http_err.code).as_str())?;
    write_line(&stream, format!("X-Error-Info: {}", http_err).as_str())?;
    write_line(&stream, "Connection: close")?;
    write_line(&stream, "Connection-Type: text/html")?;

    let error_html = format!("<html><body><h1>{}</h1></body></html>", http_err.code.get_desc());
    write_body(&stream, error_html.as_str())?;

    Ok(())
}

