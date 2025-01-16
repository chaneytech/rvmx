pub mod directive_parsers;
pub mod instruction_parsers;
mod instruction_parsers_test;
pub mod label_parsers;
mod label_parsers_test;
pub mod opcode_parsers;
mod opcode_parsers_test;
pub mod operand_parsers;
mod operand_parsers_test;
pub mod program_parsers;
mod program_parsers_test;
pub mod register_parsers;
mod register_parsers_test;

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}
