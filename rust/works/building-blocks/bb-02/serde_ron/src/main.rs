use ron::de::from_bytes;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    squares: i32,
}

fn main() {
    let a = Move { squares: 3 };
    let s = to_string(&a).expect("Serialization failed").into_bytes();
    println!("original: {:?}", a);
    println!("Ron as Vec<u8>: {:?}", s);
    println!("Ron as string: {}", str::from_utf8(&s).unwrap());
    let b: Move = from_bytes(&s).unwrap();
    println!("deserialized: {:?}", b);
}
