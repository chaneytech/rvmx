#[cfg(test)]
mod tests {

    use crate::assembler::{
        label_parsers::{label_declaration, label_usage},
        Token,
    };

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration("test:");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelDeclaration {
                name: "test".to_string()
            }
        );
        let result = label_declaration("test");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage("@test");
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelUsage {
                name: "test".to_string()
            }
        );
        let result = label_usage("test");
        assert_eq!(result.is_ok(), false);
    }
}
