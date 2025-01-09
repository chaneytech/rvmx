use crate::assembler::Token;
use nom::{digit, named, tag, types::CompleteStr, ws};

named!(pub register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                    reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);
