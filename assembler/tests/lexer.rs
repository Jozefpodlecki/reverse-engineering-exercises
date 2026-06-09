#[cfg(test)]
mod tests {
    use assembler::*;

    use super::*;

    #[test]
    fn test_lex_register() {
        let lexer = Lexer::new("rax", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, Token::Register("rax".to_string()));
        assert_eq!(tokens[1].value, Token::Eof);
    }

    #[test]
    fn test_lex_hex_immediate() {
        let lexer = Lexer::new("0x1234", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Immediate(0x1234));
    }

    #[test]
    fn test_lex_decimal_immediate() {
        let lexer = Lexer::new("1234", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Immediate(1234));
    }

    #[test]
    fn test_lex_mnemonic() {
        let lexer = Lexer::new("mov", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Mnemonic("mov".to_string()));
    }

    #[test]
    fn test_lex_memory_brackets() {
        let lexer = Lexer::new("[rsp+0x30]", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::OpenBracket);
        assert_eq!(tokens[1].value, Token::Register("rsp".to_string()));
        assert_eq!(tokens[2].value, Token::Plus);
        assert_eq!(tokens[3].value, Token::Immediate(0x30));
        assert_eq!(tokens[4].value, Token::CloseBracket);
    }

    #[test]
    fn test_lex_instruction_with_spaces() {
        let lexer = Lexer::new("mov [rsp + 0x30], r15", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Mnemonic("mov".to_string()));
        assert_eq!(tokens[1].value, Token::OpenBracket);
        assert_eq!(tokens[2].value, Token::Register("rsp".to_string()));
        assert_eq!(tokens[3].value, Token::Plus);
        assert_eq!(tokens[4].value, Token::Immediate(0x30));
        assert_eq!(tokens[5].value, Token::CloseBracket);
        assert_eq!(tokens[6].value, Token::Comma);
        assert_eq!(tokens[7].value, Token::Register("r15".to_string()));
    }

    #[test]
    fn test_lex_label() {
        let lexer = Lexer::new("start:", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Label("start".to_string()));
        assert_eq!(tokens[1].value, Token::Colon);
    }

    #[test]
    fn test_lex_comment() {
        let lexer = Lexer::new("mov rax, 0x1 ; comment here", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Mnemonic("mov".to_string()));
        assert_eq!(tokens[1].value, Token::Register("rax".to_string()));
        assert_eq!(tokens[2].value, Token::Comma);
        assert_eq!(tokens[3].value, Token::Immediate(1));
        assert_eq!(tokens[4].value, Token::Eof);
    }

    #[test]
    fn test_lex_multiple_instructions() {
        let source = "push rax\npush rcx\nmov rax, 0x12345";
        let lexer = Lexer::new(source, "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(errors.is_empty());
        assert_eq!(tokens[0].value, Token::Mnemonic("push".to_string()));
        assert_eq!(tokens[1].value, Token::Register("rax".to_string()));
        assert_eq!(tokens[2].value, Token::Mnemonic("push".to_string()));
        assert_eq!(tokens[3].value, Token::Register("rcx".to_string()));
        assert_eq!(tokens[4].value, Token::Mnemonic("mov".to_string()));
        assert_eq!(tokens[5].value, Token::Register("rax".to_string()));
        assert_eq!(tokens[6].value, Token::Comma);
        assert_eq!(tokens[7].value, Token::Immediate(0x12345));
    }

    #[test]
    fn test_lex_error_invalid_character() {
        let lexer = Lexer::new("mov @rax", "<test>");
        let (tokens, errors) = lexer.tokenize();
        
        assert!(!errors.is_empty());
        assert_eq!(errors[0].message, "Unexpected character: '@'");
    }
}