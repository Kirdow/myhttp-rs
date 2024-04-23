use std::{fs::{create_dir_all, File}, io::Write, net::TcpStream, path::{Path, PathBuf}};

use crate::{http_error::{http_errors, HttpError}, io_util::{self, validate_path}, util::{self, get_time_str}};


pub struct Transcript {
    file: File,
    prefix: Option<String>
}

impl Transcript {
    pub fn new(stream: &TcpStream) -> Result<Self, HttpError> {
        let stream_name = io_util::get_stream_name(stream);
        // Escape name
        let stream_file_name = stream_name.replace(".", "_").replace(":", "_");
        let current_time_int = util::get_time(0);

        Ok(Self {
            file: Self::try_get_file_name(&stream_file_name, current_time_int)?,
            prefix: Some(stream_name)
        })
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = if prefix.is_empty() {
            None
        } else {
            Some(prefix.to_string())
        }
    }

    pub fn get_prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }

    fn try_get_file_name(name: &String, time_int: i32) -> Result<File, HttpError> {
        let mut counter = 0;
        let base_path = PathBuf::from("./logs");
        if !base_path.exists() {
            create_dir_all(&base_path).map_err(|_| http_errors::msg::internal_server_error("Failed to create logs directory"))?;
        }

        loop {
            let path = if counter > 0 {
                PathBuf::from(format!("{}_{}_{}.log", name, time_int, counter))
            } else {
                PathBuf::from(format!("{}_{}.log", name, time_int))
            };

            let mut full_path = PathBuf::new();
            full_path.push(&base_path);
            full_path.push(path);
            
            if !full_path.exists() {
                return File::create(&full_path).map_err(|e| {
                    println!("Failed to create transcript file: {}", e);
                    http_errors::msg::internal_server_error(format!("Failed to create transcript file: {:?}", &full_path).as_str())
                });
            }

            counter += 1;
        }
    }

    pub fn push(&self, line: &str) -> Result<(), HttpError> {
        let now = get_time_str(false, true);

        if !line.is_empty() {
            for split_line in line.lines() {
                self.push_int(&now, split_line)?;
            }
        } else {
            self.push_int(&now, "")?;
        }

        Ok(())
    }

    fn push_int(&self, time: &String, line: &str) -> Result<(), HttpError> {
        let data = if let Some(prefix) = &self.prefix {
            format!("{} [{}]: {}", prefix, time, line)
        } else {
            format!("[{}]: {}", time, line)
        };

        println!("[TS] {}", data);
        write!(&self.file, "{}\r\n", data).map_err(|e| {
            http_errors::msg::internal_server_error(format!("Failed to push transcript line: {}", e).as_str())
        })
    }

    pub fn flush(&mut self) -> Result<(), HttpError> {
        self.file.flush().map_err(|e| {
            http_errors::msg::internal_server_error(format!("Failed to flush transcript file: {}", e).as_str())
        })
    }
}