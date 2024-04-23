use std::{collections::HashMap, fmt};

use crate::http_error::{HttpError, http_errors};

pub struct HttpHeaders {
    pub map: HashMap<String, String>
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add_from_line(&mut self, line: &str) -> Result<(), HttpError> {
        let mut parts = line.splitn(2, ':');

        let name = parts.next()
                                .ok_or_else(|| {
                                    http_errors::msg::bad_request("Failed to fetch header name")
                                        .set_info("Malformed header")
                                })?.trim().to_string();

        let data = parts.next()
                                .ok_or_else(|| {
                                    http_errors::msg::bad_request("Failed to fetch header data")
                                        .set_info("Malformed header")
                                })?.trim().to_string();

        self.map.insert(name, data);
        Ok(())
    }

    pub fn add_from_pair(&mut self, name: &str, value: &str) -> Result<(), HttpError> {
        if Self::is_restricted_header(name) {
            return Ok(());
        }

        self.map.insert(name.to_string(), value.to_string());
        Ok(())
    }

    pub fn add_from_pair_value<T>(&mut self, name: &str, value: &T) -> Result<(), HttpError> where T: ToString {
        if Self::is_restricted_header(name) {
            return Ok(());
        }

        self.map.insert(name.to_string(), value.to_string());
        Ok(())
    }

    pub fn get_from_name(&self, name: &String) -> Option<String> {
        self.map.get(name).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.map.iter()
    }

    // Static
    pub fn is_restricted_header(name: &str) -> bool {
        name == "Content-Type" || name == "Content-Length"
    }
}

impl fmt::Display for HttpHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpHeaders[[\r\n")?;
    
        for (key, value) in self.iter() {
            write!(f, "  {}: {}\r\n", key, value)?;
        }

        write!(f, "]]")
    }
}