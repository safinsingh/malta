use clap::Clap;
use colored::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fmt;
use std::fs;

mod checks;
mod crypto;

#[derive(Clap)]
#[clap(version = "0.1", author = "Safin S. <safinsingh.dev@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "conf.yaml")]
    config: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.1", author = "Safin S. <safinsingh.dev@gmail.com>")]
    Score,
    Encrypt,
    Decrypt,
    GenKey,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    title: String,
    records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    message: String,
    identifier: String,
    points: i16,
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
    points: i16,
}

impl fmt::Display for RepRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.points > 0 {
            write!(
                f,
                "{} ({}) - {} points",
                self.message,
                self.identifier,
                format!("{}", self.points).green().bold()
            )
        } else {
            write!(
                f,
                "{} ({}) - {} points",
                self.message,
                self.identifier,
                format!("{}", -1 * self.points).red().bold()
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
    UserInGroup(checks::UserInGroup),
    Firewall(checks::Firewall),
    Service(checks::Service),
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
            Self::GroupExists,
            Self::UserInGroup,
            Self::Firewall,
            Self::Service
        );
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Encrypt => {
            let raw =
                fs::read_to_string(opts.config).expect("There was an error reading the config");
            crypto::compress(raw);
        }
        SubCommand::Decrypt => {
            let config = crypto::decompress();
            println!("{}", config);
        }
        SubCommand::GenKey => {
            let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
            println!("{:?}", random_bytes);
        }
        SubCommand::Score => {
            let config = crypto::decompress();
            let config: Config = serde_yaml::from_str(config.as_str())
                .expect("There was an error deserializing the config");

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

            println!("{}", format!("[ -- {} -- ]", config.title).blue().bold());
            println!(
                "You have: {}",
                format!(
                    "{} vulns, {} points\n",
                    format!("{}", count).green().bold(),
                    format!("{}", score).green().bold()
                )
            );
            if count != 0 {
                println!("{}", format!("[ -- {} -- ]", "VULNS").green().bold());
                for rec in rep.iter() {
                    if &rec.points > &0 {
                        println!("{}", &rec);
                    }
                }
                println!("{}", format!("\n[ -- {} -- ]", "PENALTIES").red().bold());
                for rec in rep.into_iter() {
                    if rec.points < 0 {
                        println!("{}", rec);
                    }
                }
            }
        }
    }
}
