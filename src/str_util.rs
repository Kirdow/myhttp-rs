
pub struct Builder {
    pub result: String,
    pub delimiter: String
}

impl Builder {
    pub fn new(delimiter: &String) -> Self {
        Self {
            result: String::new(),
            delimiter: delimiter.to_owned()
        }
    }

    pub fn append(&mut self, value: &String) -> &mut Self {
        if !self.result.is_empty() {
            self.result.push_str(&self.delimiter);
        }

        self.result.push_str(value);
        self
    }

    pub fn try_append(&mut self, opt: &Option<String>) -> &mut Self {
        if let Some(value) = opt {
            self.append(value)
        } else {
            self
        }
    }

    pub fn get(&self, require_written: bool) -> Option<String> {
        if !require_written || !self.result.is_empty() {
            Some(self.result.to_owned())
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.result.is_empty()
    }
}