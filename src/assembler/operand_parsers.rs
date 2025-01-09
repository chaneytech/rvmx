use crate::assembler::Token;
use nom::types::CompleteStr;
use nom::{digit, named, tag, ws};

named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand {value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);
