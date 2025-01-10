#[cfg(test)]
mod tests {
    use crate::assembler::opcode_parsers::opcode_load;
    use crate::assembler::Token;
    use crate::instruction::Opcode;
    use nom::types::CompleteStr;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}
