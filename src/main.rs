#[macro_use]
extern crate clap;

mod cfg;

use cfg::parse_args;


fn main() {
    let cfg = parse_args();
    println!("Configuration: {:?}", cfg);
}
