use nom::error::VerboseError;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    pub scheme: Scheme,
    pub authority: Option<Authority<'a>>,
    pub resource: Resource,
    pub port: Option<Port>,
    pub path: Option<Path<'a>>,
    pub query: Option<QueryParams<'a>>,
    pub fragment: Option<Fragment<'a>>,
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
