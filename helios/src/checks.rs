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
        if let Ok(c) = fs::read_to_string(&self.file) {
            if let Ok(re) = Regex::new(&self.contains) {
                if re.is_match(&c) {
                    return true;
                }
            }
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

        if let Ok(code) = Command::new(cmd).args(args).stdout(Stdio::null()).status() {
            if let Some(e) = code.code() {
                if let Some(c) = self.code {
                    if e == c {
                        return true;
                    }
                } else {
                    if e == 0 {
                        return true;
                    }
                }
            }
        }
        false
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

        if let Ok(out) = Command::new(cmd).args(args).output() {
            if let Ok(re) = Regex::new(&self.contains) {
                if let Ok(s) = str::from_utf8(&out.stdout) {
                    if re.is_match(s) {
                        return true;
                    }
                }
            }
        }
        false
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
        }
        false
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
        }
        false
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
        false
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Firewall {}

impl Firewall {
    #[cfg(target_os = "linux")]
    pub fn query(&self) -> bool {
        let ufw = CommandOutput {
            command: "ufw status".into(),
            contains: "inactive".into(),
        };
        if !ufw.query() {
            return true;
        }
        false
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    service: String,
}

impl Service {
    #[cfg(target_os = "linux")]
    pub fn query(&self) -> bool {
        let ctl = "systemctl is-active ".to_string() + &self.service;
        let cmd = CommandOutput {
            command: ctl.into(),
            contains: "inactive".into(),
        };
        if !cmd.query() {
            return true;
        }
        false
    }
}
