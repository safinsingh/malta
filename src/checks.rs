use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::str;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandOutput {
    command: String,
    output: String,
}

impl CommandOutput {
    pub fn query(&self) -> bool {
        let mut args = self.command.split(' ');
        let cmd = args.next().unwrap();

        let out = Command::new(cmd).args(args).output();
        let out = match out {
            Ok(o) => o,
            Err(_) => return false,
        };

        let re = Regex::new(&self.output);
        let re = match re {
            Ok(r) => r,
            Err(_) => return false,
        };

        let stdout = str::from_utf8(&out.stdout);
        if let Ok(s) = stdout {
            if re.is_match(s) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}
