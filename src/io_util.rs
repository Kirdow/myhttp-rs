use std::{fs::File, io::Read, io::Write, net::TcpStream};

pub fn write_line(mut stream: &TcpStream, line: &str) -> std::io::Result<()> {
    println!("<-- {}", line);
    writeln!(stream, "{}\r", line)
}

pub fn write_body(stream: &TcpStream, body: &str) -> std::io::Result<()> {
    let len = body.len();

    write_line(stream, format!("Content-Length: {}", len).as_str())?;
    write_line(stream, "")?;
    write_line(stream, body)
}

pub fn read_all_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn write_error(stream: &TcpStream, code: i32) -> std::io::Result<()> {
    if code == 403 {
        write_line(&stream, "HTTP/1.1 403 Forbidden")?;
    } else if code == 404 {
        write_line(&stream, "HTTP/1.1 404 Not Found")?;
    } else if code == 500 {
        write_line(&stream, "HTTP/1.1 500 Internal Server Error")?;
    } else {
        write_line(&stream, "HTTP/1.1 501 Not Implemented")?;
    }

    write_line(&stream, "Connection: close")?;
    write_line(&stream, "Connection-Type: text/html")?;

    let error_str = match code {
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Not Implemented",
    };

    let error_html = format!("<html><body><h1>{}</h1></body></html>", error_str);
    write_body(&stream, error_html.as_str())?;

    Ok(())
}

