use nom::{combinator::map, multi::many1, IResult};

use super::instruction_parsers::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

pub fn program(input: &str) -> IResult<&str, Program> {
    map(
        many1(instruction_one),
        |instructions: Vec<AssemblerInstruction>| Program { instructions },
    )(input)
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}
