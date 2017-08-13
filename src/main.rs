#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yml = load_yaml!("config/clap.yml");
    let app = App::from_yaml(yml).get_matches();
}
