pub mod opcode_parsers;
mod opcode_parsers_test;
pub mod operand_parsers;
mod operand_parsers_test;
pub mod register_parsers;
mod register_parsers_test;

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
