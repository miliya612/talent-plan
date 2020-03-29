#[macro_use]
extern crate clap;
#[macro_use]
extern crate dotenv_codegen;

use clap::App;
use std::env;

fn main() {
    println!("{}", dotenv!("PORT"));

    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    }

    let yml = load_yaml!("cli.yml");
    let _ = App::from_yaml(yml).get_matches();
}
