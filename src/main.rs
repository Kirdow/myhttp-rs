mod http_util;
mod request;
mod response;
mod io_util;
mod util;
mod http_error;
mod str_util;
mod headers;
mod transcript;

use http_util::get_valid_path;
use io_util::{read_all_file, read_binary_file};
use request::HttpRequest;
use response::HttpResponse;
use transcript::Transcript;
use util::{log_title, read_line};
use http_error::{HttpError, HttpCode, http_errors};

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, BufReader, Error, Write};

use crate::io_util::{get_stream_name, write_error};

fn respond_client_error(ts: &mut Transcript, stream: &TcpStream, err: HttpError) -> io::Result<()> {
    write_error(ts, &stream, err).map_err(|e| e.convert_to(Some("Failed to send HTTP Error to client")))
}

fn end_client(mut stream: &TcpStream) -> io::Result<()> {
    stream.flush()?;
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(&stream);

    let mut request = HttpRequest::new(&stream).map_err(|e| e.convert_to(Some("Failed to create HTTP Request")))?;
    log_title(&request.transcript, "HTTP Request");
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        read_line(&mut request.transcript, line.as_str());
        if let Err(http_err) = request.feed(&line) {
            respond_client_error(&mut request.transcript, &stream, http_err)?;
            return end_client(&stream);
        }
    }

    let mut response = HttpResponse::new(request, &mut stream);
    if let Err(http_err) = response.headers.add_from_pair("Connection", "close") {
        respond_client_error(&mut response.request.transcript, &stream, http_err)?;
        return end_client(&stream);
    }

    log_title(&response.request.transcript, "HTTP Response");
    match get_valid_path(&response.request) {
        Ok(path) => {
            if response.request.resource_type == "text/html" || response.request.resource_type == "image/x-icon" {
                match read_binary_file(path.as_str()) {
                    Err(e) => response.set_error(e),
                    Ok(content) => {
                        if let Err(e) = response.set_data_response(HttpCode::E200, content) {
                            response.request.transcript.push(format!("Failed to set data response: {}", e).as_str());
                            response.set_error(e);
                        }
                    }
                }
            } else {
                response.set_error(http_errors::msg::forbidden(format!("Invalid file type: {}", response.request.path).as_str()));
            }
        },
        Err(e) => {
            response.request.transcript.push(format!("Failed to recognize requested file: {}", e).as_str());
            response.set_error(e);
        }
    }

    response.flush()?;
    end_client(&stream)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    let client_addr = stream.peer_addr();
                    if let Err(e) = handle_client(stream) {
                        eprintln!("{} Failed to handle client: {}", client_addr
                            .map(|addr| addr.to_string())
                            .unwrap_or("Unknown Address".to_string()), e);
                    }
                });
            },
            Err(e) => {
                eprintln!("Connection failed: {:?}", e);
            },
        }
    }

    Ok(())
}
