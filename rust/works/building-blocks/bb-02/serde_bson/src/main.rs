use bson;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Cursor;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    squares: i32,
}

fn main() {
    let f = File::create("bson").unwrap();

    let b: Vec<u8> = Vec::new();
    let mut c = Cursor::new(b);

    for i in 0..100 {
        let bs = bson::to_bson(&Move { squares: i }).unwrap();
        let bs = bs.as_document().unwrap();
        bson::encode_document(&mut c, bs).unwrap();
    }

    let mut writer = BufWriter::new(f);
    writer.write(c.into_inner().as_slice()).unwrap();
    match writer.flush() {
        Ok(()) => (),
        Err(e) => panic!(e),
    }

    let mut f = File::open("bson").unwrap();
    let mut b: Vec<u8> = Vec::new();
    f.read_to_end(&mut b).unwrap();
    let mut c = Cursor::new(b);
    let mut buf = vec![0; 18];

    loop {
        match c.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let d = bson::decode_document(&mut buf.as_slice()).unwrap();
                println!("decoded: {:?}", d);
            }
            Err(e) => panic!(e),
        }
    }
}
