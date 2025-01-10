#[cfg(test)]
mod tests {
    use crate::assembler::opcode_parsers::opcode_load;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_load("aold");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
