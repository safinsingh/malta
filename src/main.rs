use serde::{Deserialize, Serialize};
use serde_yaml;
use std::convert::From;
use std::fmt;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    title: String,
    records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    message: String,
    identifier: String,
    points: u8,
    checks: Vec<Check>,
}

impl Record {
    fn score(&self) -> Option<RepRecord> {
        if self.identifier.is_empty() || self.identifier.len() != 6 {
            panic!("Empty or invalid check identifier!");
        }
        let mut got = true;
        for check in &self.checks {
            if let Some(s) = &check.success {
                for cond in s.into_iter() {
                    if !cond.eval() {
                        got = false;
                    }
                }
            }
            if let Some(f) = &check.fail {
                for cond in f.into_iter() {
                    if cond.eval() {
                        got = false;
                    }
                }
            }
        }
        if got {
            return Some(RepRecord {
                message: self.message.clone(),
                identifier: self.identifier.clone(),
                points: self.points.clone(),
            });
        }
        None
    }
}

struct RepRecord {
    message: String,
    identifier: String,
    points: u8,
}

impl fmt::Display for RepRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) - {} points",
            self.message, self.identifier, self.points
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Check {
    success: Option<Vec<Vuln>>,
    fail: Option<Vec<Vuln>>,
}

#[serde(tag = "type")]
#[derive(Debug, Serialize, Deserialize)]
enum Vuln {
    FileContains(FileContains),
}

impl Vuln {
    fn eval(&self) -> bool {
        match &self {
            Vuln::FileContains(obj) => return file_contains(obj),
        }
    }
}

impl From<FileContains> for Vuln {
    fn from(check: FileContains) -> Self {
        Vuln::FileContains(check)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FileContains {
    file: String,
    contains: String,
}

fn file_contains(obj: &FileContains) -> bool {
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

fn main() {
    let config = fs::read_to_string("conf.yaml").expect("There was an error reading the config");
    let config: Config =
        serde_yaml::from_str(config.as_str()).expect("There was an error deserializing the config");
    let mut rep: Vec<RepRecord> = Vec::new();

    let mut score = 0;
    let mut count = 0;
    for rec in config.records.into_iter() {
        if let Some(r) = rec.score() {
            score += r.points;
            count += 1;
            rep.push(r);
        }
    }

    println!("{}", config.title);
    println!("{} vulns, {} points", count, score);
    if count != 0 {
        for rec in rep.into_iter() {
            println!("{}", rec);
        }
    }
}
