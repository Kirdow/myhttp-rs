mod http_util;
mod request;
mod io_util;
mod util;

use http_util::get_valid_path;
use io_util::{read_all_file, write_body, write_line};
use request::HttpRequest;
use util::{log_title, read_line};

use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, BufReader, Write};

use crate::io_util::write_error;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(&stream);

    let mut request = HttpRequest::new();

    log_title("HTTP Request");
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        read_line(line.as_str());
        request.feed(&line).expect("ERROR: Failed to feed request");
    }

    log_title("HTTP Response");
    match get_valid_path(&request) {
        Ok(path) => {
            if request.resource_type == "text/html" {
                match read_all_file(path.as_str()) {
                    Err(e) => {
                        println!("Failed to read requested file: {}", e);
                        write_error(&stream, 500)?;
                    },
                    Ok(content) => {
                        write_line(&stream, "HTTP/1.1 200 OK")?;
                        write_line(&stream, "Connection: close")?;
                        write_line(&stream, "Content-Type: text/html")?;
                        write_body(&stream, content.as_str())?;
                    }
                }
            } else {
                println!("Invalid file type: {}", path);
                write_error(&stream, 403)?;
            }
        },
        Err(e) => {
            println!("Failed to recognize requested file: {}", e);
            write_error(&stream, 404)?;
        }
    }

    stream.flush()?;
    drop(stream);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                if let Err(e) = handle_client(stream) {
                    eprintln!("Failed to handle client: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Connection failed: {:?}", e);
            },
        }
    }

    Ok(())
}
