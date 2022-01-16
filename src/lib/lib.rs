mod combinators;
mod types;

use combinators::*;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use types::{VResult, URI};

pub fn uri<'a>(input: &'a str) -> VResult<&'a str, URI<'a>> {
    map(
        tuple((
            scheme,
            opt(authority),
            resource,
            opt(port),
            opt(path),
            opt(query),
            opt(fragment),
        )),
        |(sch, aut, res, por, pat, que, fra)| URI {
            scheme: sch,
            authority: aut,
            resource: res,
            port: por,
            path: pat,
            query: que,
            fragment: fra,
        },
    )(input)
}
