#[macro_use]
extern crate clap;
#[macro_use]
extern crate dotenv_codegen;

use clap::App;

fn main() {
    println!("{}", dotenv!("PORT"));

    let yml = load_yaml!("cli.yml");
    let _ = App::from_yaml(yml).get_matches();
}
