use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandExitCode {
    command: String,
    code: Option<i32>,
}

impl CommandExitCode {
    pub fn query(&self) -> bool {
        let mut args = self.command.split(' ');
        let cmd = args.next().unwrap();

        let code = Command::new(cmd).args(args).status();
        let err = match code {
            Ok(c) => c,
            Err(_) => return false,
        };

        match err.code() {
            Some(e) => {
                if let Some(c) = self.code {
                    if e == c {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    if e == 0 {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            None => return false,
        }
    }
}
