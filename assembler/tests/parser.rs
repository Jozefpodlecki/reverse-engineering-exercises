#[cfg(test)]
mod tests {
    use assembler::{ast::Instruction, *};
    use super::*;

    #[test]
    fn test_parse_syscall() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("syscall".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Eof, location: Location { line: 1, col: 8 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], Instruction::Syscall);
    }

    #[test]
    fn test_parse_push_pop() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("push".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Register("rax".to_string()), location: Location { line: 1, col: 6 } },
            Spanned { value: Token::Mnemonic("pop".to_string()), location: Location { line: 2, col: 1 } },
            Spanned { value: Token::Register("rcx".to_string()), location: Location { line: 2, col: 5 } },
            Spanned { value: Token::Eof, location: Location { line: 2, col: 8 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_parse_mov_immediate() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("mov".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Register("rax".to_string()), location: Location { line: 1, col: 5 } },
            Spanned { value: Token::Comma, location: Location { line: 1, col: 8 } },
            Spanned { value: Token::Immediate(0x12345), location: Location { line: 1, col: 10 } },
            Spanned { value: Token::Eof, location: Location { line: 1, col: 16 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        
        assert_eq!(instructions.len(), 1);
    }

    #[test]
    fn test_parse_mov_memory() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("mov".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::OpenBracket, location: Location { line: 1, col: 5 } },
            Spanned { value: Token::Register("rsp".to_string()), location: Location { line: 1, col: 6 } },
            Spanned { value: Token::Plus, location: Location { line: 1, col: 10 } },
            Spanned { value: Token::Immediate(0x30), location: Location { line: 1, col: 12 } },
            Spanned { value: Token::CloseBracket, location: Location { line: 1, col: 16 } },
            Spanned { value: Token::Comma, location: Location { line: 1, col: 17 } },
            Spanned { value: Token::Register("r15".to_string()), location: Location { line: 1, col: 19 } },
            Spanned { value: Token::Eof, location: Location { line: 1, col: 22 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        
        assert_eq!(instructions.len(), 1);
    }

    #[test]
    fn test_parse_sub_rsp() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("sub".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Register("rsp".to_string()), location: Location { line: 1, col: 5 } },
            Spanned { value: Token::Comma, location: Location { line: 1, col: 8 } },
            Spanned { value: Token::Immediate(0x38), location: Location { line: 1, col: 10 } },
            Spanned { value: Token::Eof, location: Location { line: 1, col: 14 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        
        assert_eq!(instructions.len(), 1);
    }

    #[test]
    fn test_parse_label() {
        let tokens = vec![
            Spanned { value: Token::Label("start".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Colon, location: Location { line: 1, col: 6 } },
            Spanned { value: Token::Mnemonic("mov".to_string()), location: Location { line: 2, col: 1 } },
            Spanned { value: Token::Register("rax".to_string()), location: Location { line: 2, col: 5 } },
            Spanned { value: Token::Comma, location: Location { line: 2, col: 8 } },
            Spanned { value: Token::Immediate(1), location: Location { line: 2, col: 10 } },
            Spanned { value: Token::Eof, location: Location { line: 2, col: 11 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        
        assert_eq!(instructions.len(), 1);
    }

    #[test]
    fn test_parse_error_unknown_mnemonic() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("unknown".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Eof, location: Location { line: 1, col: 8 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_comma() {
        let tokens = vec![
            Spanned { value: Token::Mnemonic("mov".to_string()), location: Location { line: 1, col: 1 } },
            Spanned { value: Token::Register("rax".to_string()), location: Location { line: 1, col: 5 } },
            Spanned { value: Token::Register("rcx".to_string()), location: Location { line: 1, col: 9 } },
            Spanned { value: Token::Eof, location: Location { line: 1, col: 12 } },
        ];
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
    }
}