use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

use super::Token;

pub fn label_declaration(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        map(tuple((alphanumeric1::<&str, _>, tag(":"))), |(name, _)| {
            Token::LabelDeclaration {
                name: name.to_string(),
            }
        }),
    )(input)
}

pub fn label_usage(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        map(preceded(tag("@"), alphanumeric1), |name: &str| {
            Token::LabelUsage {
                name: name.to_string(),
            }
        }),
    )(input)
}
