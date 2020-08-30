use regex::Regex;
use serde::{Deserialize, Serialize};
use users::{get_group_by_name, get_user_by_name, get_user_groups};

use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
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

        let re = Regex::new(&self.contains);
        let re = match re {
            Ok(r) => r,
            Err(_) => return false,
        };

        if re.is_match(&content) {
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

        let code = Command::new(cmd).args(args).stdout(Stdio::null()).status();
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
    contains: String,
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

        let re = Regex::new(&self.contains);
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

#[derive(Debug, Deserialize, Serialize)]
pub struct FileExists {
    path: String,
}

impl FileExists {
    pub fn query(&self) -> bool {
        return Path::new(&self.path).exists();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserExists {
    user: String,
}

impl UserExists {
    #[cfg(target_os = "linux")]
    pub fn query(&self) -> bool {
        if let Some(_) = get_user_by_name(&self.user) {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupExists {
    group: String,
}

impl GroupExists {
    #[cfg(target_os = "linux")]
    pub fn query(&self) -> bool {
        if let Some(_) = get_group_by_name(&self.group) {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInGroup {
    user: String,
    group: String,
}

impl UserInGroup {
    #[cfg(target_os = "linux")]
    pub fn query(&self) -> bool {
        if let Some(id) = get_user_by_name(&self.user) {
            if let Some(groups) = get_user_groups(&self.user, id.uid()) {
                for group in groups {
                    if let Some(name) = group.name().to_str() {
                        if name == &self.group {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FirewallUp {}

impl FirewallUp {
    #[cfg(target_os = "linux")]
    pub fn query(&self) -> bool {
        let ufw = CommandOutput {
            command: "ufw status".into(),
            contains: "active".into(),
        };
        if ufw.query() {
            return true;
        }
        return false;
    }
}
