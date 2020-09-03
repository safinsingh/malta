#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate firebase_rs;
use chrono::prelude::*;
use firebase_rs::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod crypto;
mod frontend;

#[derive(Serialize, Deserialize)]
struct Config {
    title: String,
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
struct Record {
    message: String,
    identifier: String,
    points: i16,
}

#[derive(Serialize, Deserialize)]
struct Req {
    id: String,
    vulnstr: String,
    points: String,
}

#[post("/", format = "json", data = "<req>")]
fn vuln_post(req: Json<Req>) {
    let mut vulns = HashMap::new();
    let config = crypto::decompress();
    let config: Config = serde_yaml::from_str(config.as_str())
        .expect("There was an error deserializing the config!");

    for rec in config.records.into_iter() {
        vulns.insert(rec.identifier, rec.message);
    }

    let vuln_arr = req
        .vulnstr
        .as_bytes()
        .chunks(6)
        .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) });

    let mut ret: Vec<String> = Vec::new();
    for v in vuln_arr.into_iter() {
        if let Some(i) = vulns.get(v) {
            ret.push(i.into());
        }
    }

    let firebase = Firebase::new("https://malta-rs.firebaseio.com").unwrap();
    let loc = firebase.at(&req.id).unwrap();
    let time = Local::now().timestamp();

    loc.push(&format!(
        "{{\"points\":{},\"vulns\":{:?},\"time\":{}}}",
        &req.points, ret, time
    ))
    .unwrap();

    println!("ID: {}\nPoints: {}\nVulns:\n{:?}", req.id, req.points, ret);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![vuln_post, frontend::front])
        .launch();
}
