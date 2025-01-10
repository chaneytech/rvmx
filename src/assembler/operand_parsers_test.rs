#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;

    use crate::assembler::{operand_parsers::integer_operand, Token};

    #[test]
    fn test_parse_integer_operand() {
        // Test a valid integer operand
        let result = integer_operand(CompleteStr("#10"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntegerOperand { value: 10 });

        // Test an invalid one (missing the #)
        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }
}
