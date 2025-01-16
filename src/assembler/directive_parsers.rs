use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0},
    combinator::{eof, map, map_res, opt},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use super::{instruction_parsers::AssemblerInstruction, operand_parsers::operand, Token};

pub fn directive_declaration(input: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        map_res(preceded(tag("."), alpha1), |name: &str| {
            Ok::<Token, &str>(Token::Directive {
                name: name.to_string(),
            })
        }),
    )(input)
}

pub fn directive_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    preceded(
        multispace0,
        terminated(
            map(
                tuple((
                    tag(","),
                    directive_declaration,
                    opt(operand),
                    opt(operand),
                    opt(operand),
                )),
                |(_, d, r1, r2, r3)| AssemblerInstruction {
                    opcode: None,
                    operand1: r1,
                    operand2: r2,
                    operand3: r3,
                    label: None,
                    directive: Some(d),
                },
            ),
            alt((multispace0, line_ending, eof)),
        ),
    )(input)
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    directive_combined(input)
}
