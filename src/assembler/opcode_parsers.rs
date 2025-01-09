use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;
use nom::{do_parse, named, tag};

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
      tag!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);
