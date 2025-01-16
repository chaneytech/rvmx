use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::preceded,
    IResult,
};

use super::Token;

pub fn register(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        map_res(preceded(tag("$"), digit1), |num: &str| {
            Ok::<Token, &str>(Token::Register {
                reg_num: num.parse::<u8>().unwrap(),
            })
        }),
    )(input)
}
