use super::types::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{alpha1, alphanumeric1, one_of};
use nom::character::is_alphanumeric;
use nom::combinator::{map, opt, recognize};
use nom::error::{ErrorKind, VerboseError};
use nom::multi::{count, many1, many_m_n};
use nom::sequence::{pair, preceded, terminated};
use nom::Err as NomErr;
use nom::{AsChar, InputTakeAtPosition};
use std::str::FromStr;

pub fn schema(input: &str) -> VResult<&str, Scheme> {
    map(
        alt((tag_no_case("http://"), tag_no_case("https://"))),
        |result: &str| result.into(),
    )(input)
}

pub fn authority(input: &str) -> VResult<&str, Authority> {
    terminated(
        pair(alphanumeric1, opt(preceded(tag(":"), alphanumeric1))),
        tag("@"),
    )(input)
}

pub fn resource(input: &str) -> VResult<&str, Resource> {
    alt((host, ip))(input)
}

pub fn port(input: &str) -> VResult<&str, Port> {
    preceded(tag(":"), port_number)(input)
}

pub fn path(input: &str) -> VResult<&str, Path> {
    map(
        many1(pair(tag("/"), opt(alt((pathchar1, hostchar1))))),
        |components: Vec<(&str, Option<&str>)>| {
            let mut result: Vec<&str> = vec![];

            for (_, possible) in components {
                if let Some(route) = possible {
                    println!("{route}");
                    result.push(route);
                }
            }

            return result;
        },
    )(input)
}

fn host(input: &str) -> VResult<&str, Resource> {
    alt((
        recognize(pair(many1(terminated(hostchar1, tag("."))), alpha1)),
        recognize(hostchar1),
    ))(input)
    .and_then(|(next, _)| {
        Ok((
            next,
            Resource::Host(input[0..(input.len() - next.len())].to_string()),
        ))
    })
}

fn customchars1<T>(input: T, validate: &dyn Fn(u8) -> bool) -> VResult<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| validate(item.as_char() as u8),
        ErrorKind::AlphaNumeric,
    )
}

fn hostchar1(input: &str) -> VResult<&str, &str> {
    customchars1(input, &is_hostchar)
}

fn pathchar1(input: &str) -> VResult<&str, &str> {
    customchars1(input, &is_path_char)
}

fn is_path_char(input: u8) -> bool {
    !(is_alphanumeric(input) || input == b'-' || input == b'.')
}

fn is_hostchar(input: u8) -> bool {
    !(is_alphanumeric(input) || input == b'-')
}

fn ip(input: &str) -> VResult<&str, Resource> {
    map(
        pair(ip_number, count(preceded(tag("."), ip_number), 3)),
        |(first, rest): (u8, Vec<u8>)| Resource::IP([first, rest[0], rest[1], rest[2]]),
    )(input)
}

fn port_number(input: &str) -> VResult<&str, u16> {
    custom_number(input, 5)
}

fn ip_number(input: &str) -> VResult<&str, u8> {
    custom_number(input, 3)
}

fn custom_number<T: FromStr>(input: &str, max: usize) -> VResult<&str, T> {
    match many_m_n(1, max, one_of("0123456789"))(input) {
        Ok((next, list)) => {
            let list: String = list.into_iter().collect();
            match list.parse::<T>() {
                Ok(parsed) => Ok((next, parsed)),
                Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema() {
        assert!(schema("ftp://").is_err());
        assert!(schema("hhttp://").is_err());
        assert_eq!(Ok(("", Scheme::HTTP)), schema("http://"));
        assert_eq!(Ok(("", Scheme::HTTPS)), schema("https://"));
    }

    #[test]
    fn test_authority() {
        assert!(authority(":@page.com").is_err());
        assert!(authority("username:@page.com").is_err());
        assert!(authority(":password@page.com").is_err());
        assert_eq!(
            authority("username@page.com"),
            Ok(("page.com", ("username", None)))
        );
        assert_eq!(
            authority("username:password@page.com"),
            Ok(("page.com", ("username", Some("password"))))
        );
    }

    #[test]
    fn test_host() {
        assert!(host(".com").is_err());
        assert!(host("$$$.com").is_err());
        assert_eq!(
            host("localhost:8080"),
            Ok((":8080", Resource::Host("localhost".to_string())))
        );
        assert_eq!(
            host("example.org:8080"),
            Ok((":8080", Resource::Host("example.org".to_string())))
        );
        assert_eq!(
            host("some-subsite.example.org:8080"),
            Ok((
                ":8080",
                Resource::Host("some-subsite.example.org".to_string())
            ))
        );
        assert_eq!(
            host("example.123"),
            Ok((".123", Resource::Host("example".to_string())))
        );
    }

    #[test]
    fn test_ip() {
        assert!(ip("192.168.0:8080").is_err());
        assert!(ip("999.168.0.0:8080").is_err());
        assert!(ip("1924.168.0.1:8080").is_err());
        assert!(ip("192.168.0000.144:8080").is_err());
        assert_eq!(
            ip("0.0.0.0:8080"),
            Ok((":8080", Resource::IP([0, 0, 0, 0])))
        );
        assert_eq!(
            ip("192.168.0.1444:8080"),
            Ok(("4:8080", Resource::IP([192, 168, 0, 144])))
        );
        assert_eq!(
            ip("192.168.0.1:8080"),
            Ok((":8080", Resource::IP([192, 168, 0, 1])))
        );
    }

    #[test]
    fn test_port() {
        assert!(port("8080").is_err());
        assert!(port(".8080").is_err());
        assert!(port(":808080").is_err());
        assert_eq!(port(":8"), Ok(("", 8)));
        assert_eq!(port(":8080"), Ok(("", 8080)));
    }

    #[test]
    fn test_path() {
        assert_eq!(path("/a/b/c?d"), Ok(("?d", vec!["a", "b", "c"])));
        assert_eq!(path("/a/b/c/?d"), Ok(("?d", vec!["a", "b", "c"])));
        assert_eq!(path("/a/b-c-d/c/?d"), Ok(("?d", vec!["a", "b-c-d", "c"])));
        assert_eq!(path("/a/1234/c/?d"), Ok(("?d", vec!["a", "1234", "c"])));
        assert_eq!(
            path("/a/1234/c.txt?d"),
            Ok(("?d", vec!["a", "1234", "c.txt"]))
        );
    }
}
