use crate::http_util::split_method;

pub struct HttpRequest {
    pub path: String,
    pub resource_type: String,
    pub version: String,
    pub valid: bool,
    pub is_init: bool,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            resource_type: String::new(),
            version: String::new(),
            valid: false,
            is_init: false
        }
    }

    pub fn get_file_name(&self) -> String {
        if self.path.ends_with("/") {
            format!("{}index.html", self.path)
        } else {
            self.path.clone()
        }
    }

    pub fn init(&mut self, input: &String) -> Result<(), String> {
        let (method, path, version) = split_method(input.as_str()).expect("Failed to parse data");

        if method == "GET" {
            println!("GET Request");
            println!("Path: {}", path);
        } else {
            return Err("Unknown HTTP method".to_string());
        }

        self.path = if path.ends_with("/") {
            format!("{}index.html", path)
        } else {
            path.clone()
        };

        if version != "HTTP/1.1" {
            return Err(format!("HTTP version {} is unsupported", version));
        } else {
            self.version = version;
        }

        self.valid = true;
        self.is_init = true;

        Ok(())
    }

    pub fn feed(&mut self, input: &String) -> Result<(), String> {
        if !self.is_init {
            return self.init(&input);
        }

        Ok(())
    }
}
