use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, TimeZone, Utc};

use crate::{str_util::Builder, transcript::Transcript, http_error::HttpError};

pub fn log_empty(ts: &Transcript, n: i32) -> Result<(), HttpError> {
    for _ in 0..n {
        ts.push("")?;
    }

    Ok(())
}

pub fn log_title(ts: &Transcript, title: &str) -> Result<(), HttpError> {
    log_empty(ts, 2)?;
    ts.push(title)
}

pub fn read_line(ts: &mut Transcript, line: &str) -> Result<(), HttpError> {
    ts.with_prefix("-->", |ts| ts.push(line))
}

#[allow(unused)]
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

    get_time_str_from(&now, date, time)
}

pub fn get_time_str_from<T: TimeZone>(date_time: &DateTime<T>, date: bool, time: bool) -> String where T::Offset: std::fmt::Display {
    let mut result = Builder::new(&String::from(" "));

    if date {
        let formatted = date_time.format("%Y-%m-%d").to_string();
        result.append(&formatted);
    }

    if time {
        let formatted = date_time.format("%H:%M:%S%.3f").to_string();
        result.append(&formatted);
    }

    return result.result;
}
