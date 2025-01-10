use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::IResult;

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        map_res(alpha1, |s: &str| {
            Ok::<Token, &str>(Token::Op {
                code: Opcode::from(s.to_uppercase().as_str()),
            })
        }),
    )(input)
}
