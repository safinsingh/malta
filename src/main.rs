use std::convert::From;
use std::fs;

struct Config {
    title: String,
    vulns: Vec<Record>,
}

#[derive(Debug)]
struct Record {
    message: String,
    points: u8,
    checks: Vec<Check>,
}

impl Record {
    fn score(&self) -> Option<RepRecord> {
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
                points: self.points.clone(),
            });
        }
        None
    }
}

#[derive(Debug)]
struct RepRecord {
    message: String,
    points: u8,
}

#[derive(Debug)]
struct Check {
    success: Option<Vec<Vuln>>,
    fail: Option<Vec<Vuln>>,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    let mut rep: Vec<RepRecord> = Vec::new();
    let mut records: Vec<Record> = Vec::new();

    records.push(Record {
        message: "FileContains".into(),
        points: 10,
        checks: vec![Check {
            success: Some(vec![FileContains {
                file: "/home/safin/Documents/helios/hi.txt".into(),
                contains: "hello".into(),
            }
            .into()]),
            fail: None,
        }],
    });

    let mut score = 0;
    let mut count = 0;
    for rec in records.into_iter() {
        if let Some(r) = rec.score() {
            score += r.points;
            count += 1;
            rep.push(r);
        }
    }

    println!("{} vulns, {} points", count, score);
    if count != 0 {
        println!("{:?}", rep);
    }
}
