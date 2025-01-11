use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::preceded,
    IResult,
};

use super::Token;

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        map_res(preceded(tag("#"), digit1), |num: &str| {
            Ok::<Token, &str>(Token::IntegerOperand {
                value: num.parse::<i32>().unwrap(),
            })
        }),
    )(input)
}
