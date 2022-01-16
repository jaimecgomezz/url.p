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

pub fn hostchar1<T>(input: T) -> VResult<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| is_hostchar(item.as_char() as u8),
        ErrorKind::AlphaNumeric,
    )
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
    fn schema_t() {
        assert!(schema("ftp://").is_err());
        assert!(schema("hhttp://").is_err());
        assert_eq!(Ok(("", Scheme::HTTP)), schema("http://"));
        assert_eq!(Ok(("", Scheme::HTTPS)), schema("https://"));
    }

    #[test]
    fn authority_t() {
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
    fn host_t() {
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
    fn ip_t() {
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
}
