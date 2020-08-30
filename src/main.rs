use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt;
use std::fs;

mod checks;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    title: String,
    records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    message: String,
    identifier: String,
    points: i8,
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
    points: i8,
}

impl fmt::Display for RepRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.points > 0 {
            write!(
                f,
                "{} ({}) - {} points",
                self.message, self.identifier, self.points
            )
        } else {
            write!(
                f,
                "[PENALTY] {} ({}) - {} points",
                self.message, self.identifier, self.points
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Check {
    success: Option<Vec<Vuln>>,
    fail: Option<Vec<Vuln>>,
}

// To add a check type, create a struct and function
// for it in checks.rs and add it to this enum
// then, add it to the Vuln impl eval
// generation macro like so: Self::Struct
#[serde(tag = "type")]
#[derive(Debug, Serialize, Deserialize)]
enum Vuln {
    FileContains(checks::FileContains),
    CommandExitCode(checks::CommandExitCode),
    CommandOutput(checks::CommandOutput),
    FileExists(checks::FileExists),
    UserExists(checks::UserExists),
    GroupExists(checks::GroupExists),
}

macro_rules! gen_evals {
($type:expr,$($variant:path),+) => {
        match $type {
            $(
                $variant(c) => return c.query(),
            )+
        }
    }
}

impl Vuln {
    fn eval(&self) -> bool {
        gen_evals!(
            self,
            Self::FileContains,
            Self::CommandExitCode,
            Self::CommandOutput,
            Self::FileExists,
            Self::UserExists,
            Self::GroupExists
        );
    }
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
