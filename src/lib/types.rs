use nom::error::VerboseError;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    resource: Resource,
    port: Option<Port>,
    path: Option<Path<'a>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<Fragment<'a>>,
}

impl URI<'static> {
    pub fn new() -> Self {
        URI {
            scheme: Scheme::HTTP,
            authority: Some(("username", Some("password"))),
            resource: Resource::IP([127, 0, 0, 1]),
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
pub enum Resource {
    Host(String),
    IP([u8; 4]),
}

pub type Port = u16;

pub type Path<'a> = Vec<&'a str>;

pub type Fragment<'a> = &'a str;

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

pub type VResult<I, O> = IResult<I, O, VerboseError<I>>;
