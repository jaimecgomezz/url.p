use super::types::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::alphanumeric1;
use nom::combinator::{map, opt};
use nom::sequence::{pair, preceded, terminated};

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
}
