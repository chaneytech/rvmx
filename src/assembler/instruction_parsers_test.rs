#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;

    use crate::{
        assembler::{
            instruction_parsers::{instruction_one, AssemblerInstruction},
            Token,
        },
        instruction::Opcode,
    };

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}
