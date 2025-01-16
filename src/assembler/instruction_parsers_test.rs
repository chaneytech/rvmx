#[cfg(test)]
mod tests {

    use crate::{
        assembler::{
            instruction_parsers::{instruction, AssemblerInstruction},
            Token,
        },
        instruction::Opcode,
    };

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction("load $0 #100\n");
        println!("{:?}", result);
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None,
                    label: None,
                    directive: None,
                }
            ))
        );
    }
}
