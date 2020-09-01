use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn compress(config: String) {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    if let Ok(_) = e.write_all(config.as_bytes()) {
        if let Ok(b) = e.finish() {
            if let Ok(mut f) = File::create("conf.z") {
                if let Err(_) = f.write_all(&crypt(b)) {
                    panic!("Failed to write compressed bytes to file!");
                }
            }
        } else {
            panic!("Failed to get compressed bytes!");
        }
    } else {
        panic!("Error writing to zlib buffer!");
    }
}

pub fn decompress() -> String {
    if let Ok(conf) = fs::read("conf.z") {
        let dec = crypt(conf);
        let mut d = ZlibDecoder::new(dec.as_slice());
        let mut s = String::new();

        if let Ok(_) = d.read_to_string(&mut s) {
            format!("{}", s)
        } else {
            panic!(
                "Failed to read zlib decompressed data to string! Your config is likely corrupted!"
            );
        }
    } else {
        panic!("Failed to read compressed data!");
    }
}

fn crypt(input: Vec<u8>) -> Vec<u8> {
    // Replace these keys with values generated
    // by `helios gen-keys`

    let key1: Vec<u8> = vec![
        9, 112, 230, 49, 125, 19, 205, 183, 213, 237, 183, 183, 150, 165, 39, 243, 254, 101, 90,
        157, 228, 136, 252, 124, 243, 9, 28, 155, 57, 96, 231, 187,
    ];
    let key2: Vec<u8> = vec![
        149, 86, 89, 244, 199, 198, 133, 208, 139, 159, 28, 229, 143, 18, 233, 83, 126, 26, 182,
        181, 224, 222, 109, 107, 134, 207, 219, 140, 176, 1, 235, 172,
    ];

    let key3: Vec<u8> = key1
        .iter()
        .zip(key2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    let out: Vec<u8> = input
        .iter()
        .zip(key3.iter().cycle())
        .map(|(&x2, &x1)| x1 ^ x2)
        .collect();

    out
}
