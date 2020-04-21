use bson;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Cursor;
use std::io::SeekFrom;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    squares: i32,
}

fn main() {
    main_with_vec8()
}

fn main_with_vec8() {
    let b: Vec<u8> = Vec::new();
    let mut c = Cursor::new(b);

    for i in 0..100 {
        let bs = bson::to_bson(&Move { squares: i }).unwrap();
        let bs = bs.as_document().unwrap();
        bson::encode_document(&mut c, bs).unwrap();
    }

    // TODO: handle the size with better way
    let mut buf = vec![0; 18];

    c.seek(SeekFrom::Start(0)).unwrap();
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

fn main_with_file() {
    let f = File::create("bson").unwrap();
    let mut writer = BufWriter::new(f);

    for i in 0..100 {
        let bs = bson::to_bson(&Move { squares: i }).unwrap();
        let bs = bs.as_document().unwrap();
        bson::encode_document(&mut writer, bs).unwrap();
    }

    match writer.flush() {
        Ok(()) => (),
        Err(e) => panic!(e),
    }

    let mut f = File::open("bson").unwrap();
    // TODO: handle the size with better way
    let mut buf = vec![0; 18];

    loop {
        match f.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let d = bson::decode_document(&mut buf.as_slice()).unwrap();
                println!("decoded: {:?}", d);
            }
            Err(e) => panic!(e),
        }
    }
}

