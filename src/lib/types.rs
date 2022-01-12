use nom::error::VerboseError;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<&'a str>,
}

impl URI<'static> {
    pub fn new() -> Self {
        URI {
            scheme: Scheme::HTTP,
            authority: Some(("username", Some("password"))),
            host: Host::IP([127, 0, 0, 1]),
            port: Some(80),
            path: Some(vec!["some", "important", "path"]),
            query: Some(vec![("a", "1")]),
            fragment: Some("fragment"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

impl From<&str> for Scheme {
    fn from(s: &str) -> Self {
        match s {
            "http://" | "HTTP://" => Scheme::HTTP,
            "https://" | "HTTPS://" => Scheme::HTTPS,
            _ => panic!("Unsupported scheme."),
        }
    }
}

pub type Authority<'a> = (&'a str, Option<&'a str>);

#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    Host(String),
    IP([u8; 4]),
}

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

pub type VResult<I, O> = IResult<I, O, VerboseError<I>>;
