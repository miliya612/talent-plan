use bson;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    squares: i32,
}

fn main() {
    let b = bson::to_bson(&Move { squares: 1 }).unwrap();
    let b = b.as_document().unwrap();
    println!("bson: {:?}", b);
    let mut frw = File::create("bson").unwrap();
    bson::encode_document(&mut frw, b).unwrap();
    let fro = File::open("bson").unwrap();
    let mut r = BufReader::new(fro);
    let d = bson::decode_document(&mut r).unwrap();
    println!("decoded: {:?}", d);
}
