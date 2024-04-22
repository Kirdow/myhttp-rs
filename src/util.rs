pub fn log_empty(n: i32) {
    for _ in 0..n {
        print!("\n");
    }
}

pub fn log_title(who: &str, title: &str) {
    log_empty(2);
    println!("{} {}", who, title);
}

pub fn read_line(who: &str, line: &str) {
    println!("{} --> {}", who, line);
}
