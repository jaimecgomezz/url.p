use super::types::*;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::map;

pub fn schema(input: &str) -> VResult<&str, Scheme> {
    map(
        alt((tag_no_case("http://"), tag_no_case("https://"))),
        |result: &str| result.into(),
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
}
