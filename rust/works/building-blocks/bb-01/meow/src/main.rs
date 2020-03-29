#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yml = load_yaml!("cli.yml");
    let _ = App::from_yaml(yml).get_matches();
}
