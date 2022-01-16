extern crate urlp_lib;

use std::env;
use urlp_lib::uri;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let parsable = args.get(1).expect("USAGE: urlp <url>");

    println!("{:#?}", uri(parsable));
}
