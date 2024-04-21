use std::net::{TcpListener, TcpStream};
use std::io::{self, BufReader, BufRead, Write};

fn write_body(mut stream: &TcpStream, body: &str) -> std::io::Result<()> {
    let len = body.len();
    writeln!(stream, "Content-Length: {}\r", len)?;
    writeln!(stream, "\r")?;
    writeln!(stream, "{}\r", body)
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(&stream);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        println!("{}", line);
    }

    stream.write_all("HTTP/1.1 200 OK\r\n".as_bytes())?;
    stream.write_all("Connection: close\r\n".as_bytes())?;
    stream.write_all("Content-Type: text/html\r\n".as_bytes())?;
    write_body(&stream, "<html><body><h1>Hello, World!</h1></body></html>")?;
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
