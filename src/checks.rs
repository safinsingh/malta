use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct FileContains {
    file: String,
    contains: String,
}

pub fn file_contains(obj: &FileContains) -> bool {
    let content = fs::read_to_string(&obj.file);
    let content = match content {
        Ok(s) => s,
        Err(_) => return false,
    };

    if content.contains(&obj.contains) {
        return true;
    }
    false
}
