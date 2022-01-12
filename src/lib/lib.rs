#[allow(dead_code)]
mod combinators;
mod types;

use types::{VResult, URI};

pub fn uri<'a>(input: &'a str) -> VResult<&'a str, URI<'a>> {
    Ok((input, URI::new()))
}
