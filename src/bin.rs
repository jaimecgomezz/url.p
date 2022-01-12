extern crate urlp_lib;

use std::env;
use urlp_lib::uri;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let parsable = match args.get(1) {
        Some(found) => found.as_str(),
        None => "https://username:password@host:80/some/path?a=1&b=two&c=3#fragment",
    };

    println!("{:#?}", uri(parsable));
}
