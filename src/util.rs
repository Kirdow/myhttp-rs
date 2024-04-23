use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

use crate::{str_util::Builder, transcript::Transcript};

pub fn log_empty(mut ts: &Transcript, n: i32) {
    for _ in 0..n {
        ts.push("");
    }
}

pub fn log_title(mut ts: &Transcript, title: &str) {
    log_empty(ts, 2);
    ts.push(title);
}

pub fn read_line(mut ts: &Transcript, line: &str) {
    ts.push(format!("--> {}", line).as_str());
}

pub fn get_time(default: i32) -> i32 {
    let start = SystemTime::now();
    if let Ok(duration_since_epoch) = start.duration_since(UNIX_EPOCH) {
        duration_since_epoch.as_secs() as i32
    } else {
        default
    }
}

pub fn get_time_str(date: bool, time: bool) -> String {
    let now: DateTime<Utc> = Utc::now();

    let mut result = Builder::new(&String::from(" "));

    if date {
        let formatted = now.format("%Y-%m-%d").to_string();
        result.append(&formatted);
    }

    if time {
        let formatted = now.format("%H:%M:%S%.3f").to_string();
        result.append(&formatted);
    }

    return result.result;
}
