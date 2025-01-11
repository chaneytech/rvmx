use nom::{
    branch::alt,
    character::complete::{line_ending, multispace0},
    combinator::{eof, map},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use super::{
    opcode_parsers::opcode, operand_parsers::integer_operand, register_parsers::register, Token,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((instruction_two, instruction_three, instruction_one))(input)
}

pub fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    preceded(
        multispace0,
        terminated(
            map(tuple((opcode,)), |(o,)| AssemblerInstruction {
                opcode: o,
                operand1: None,
                operand2: None,
                operand3: None,
            }),
            alt((multispace0, line_ending, eof)),
        ),
    )(input)
}

pub fn instruction_two(input: &str) -> IResult<&str, AssemblerInstruction> {
    preceded(
        multispace0,
        terminated(
            map(tuple((opcode, register, integer_operand)), |(o, r, i)| {
                AssemblerInstruction {
                    opcode: o,
                    operand1: Some(r),
                    operand2: Some(i),
                    operand3: None,
                }
            }),
            alt((multispace0, line_ending, eof)),
        ),
    )(input)
}

pub fn instruction_three(input: &str) -> IResult<&str, AssemblerInstruction> {
    preceded(
        multispace0,
        terminated(
            map(
                tuple((opcode, register, register, register)),
                |(o, r1, r2, r3)| AssemblerInstruction {
                    opcode: o,
                    operand1: Some(r1),
                    operand2: Some(r2),
                    operand3: Some(r3),
                },
            ),
            alt((multispace0, line_ending, eof)),
        ),
    )(input)
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}
