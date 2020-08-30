use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct FileContains {
    file: String,
    contains: String,
}

impl FileContains {
    pub fn query(&self) -> bool {
        let content = fs::read_to_string(&self.file);
        let content = match content {
            Ok(s) => s,
            Err(_) => return false,
        };

        if content.contains(&self.contains) {
            return true;
        }
        false
    }
}
