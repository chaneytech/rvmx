use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;
use nom::{do_parse, named, tag_no_case};

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
      tag_no_case!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);
