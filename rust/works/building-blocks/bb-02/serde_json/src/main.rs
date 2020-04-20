use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    squares: i32,
}

fn main() {
    let a = Move { squares: 3 };
    let f = File::create("move.log").unwrap();
    let w = BufWriter::new(f);
    serde_json::to_writer(w, &a).unwrap();
    let f = File::open("move.log").unwrap();
    let b: Move = serde_json::from_reader(f).unwrap();
    println!("{:?}", a);
    println!("{:?}", b);
}

