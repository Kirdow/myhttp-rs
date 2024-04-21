pub fn log_empty(n: i32) {
    for _ in 0..n {
        print!("\n");
    }
}

pub fn log_title(title: &str) {
    log_empty(2);
    println!("{}", title);
}

pub fn read_line(line: &str) {
    println!("--> {}", line);
}
