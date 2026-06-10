mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_and_reg_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("and rax, rbx").unwrap();
        assert_eq!(result, vec![0x48, 0x21, 0xD8]);
        assert_ice(&result, iced_x86::Code::And_rm64_r64);
    }

    #[test]
    fn test_and_rax_imm8() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("and rax, 0x7F").unwrap();
        assert_ice(&result, iced_x86::Code::And_rm64_imm8);
    }

    #[test]
    fn test_or_reg_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("or rax, rbx").unwrap();
        assert_eq!(result, vec![0x48, 0x09, 0xD8]);
        assert_ice(&result, iced_x86::Code::Or_rm64_r64);
    }

    #[test]
    fn test_or_rax_imm8() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("or rax, 0x7F").unwrap();
        assert_ice(&result, iced_x86::Code::Or_rm64_imm8);
    }

    #[test]
    fn test_xor_reg_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("xor rax, rbx").unwrap();
        assert_eq!(result, vec![0x48, 0x31, 0xD8]);
        assert_ice(&result, iced_x86::Code::Xor_rm64_r64);
    }

    #[test]
    fn test_xor_rax_rax() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("xor rax, rax").unwrap();
        assert_eq!(result, vec![0x48, 0x31, 0xC0]);
        assert_ice(&result, iced_x86::Code::Xor_rm64_r64);
    }

    #[test]
    fn test_xor_rax_imm8() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("xor rax, 0x7F").unwrap();
        assert_ice(&result, iced_x86::Code::Xor_rm64_imm8);
    }

    #[test]
    fn test_not_rax() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("not rax").unwrap();
        assert_eq!(result, vec![0x48, 0xF7, 0xD0]);
        assert_ice(&result, iced_x86::Code::Not_rm64);
    }

    #[test]
    fn test_neg_rax() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("neg rax").unwrap();
        assert_eq!(result, vec![0x48, 0xF7, 0xD8]);
        assert_ice(&result, iced_x86::Code::Neg_rm64);
    }

    #[test]
    fn test_test_reg_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("test rax, rbx").unwrap();
        assert_eq!(result, vec![0x48, 0x85, 0xD8]);
        assert_ice(&result, iced_x86::Code::Test_rm64_r64);
    }

    #[test]
    fn test_test_rax_imm32() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("test rax, 0x1234").unwrap();
        assert_ice(&result, iced_x86::Code::Test_rm64_imm32);
    }

    #[test]
    fn test_and_memory_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("and [rax], rbx").unwrap();
        assert_ice(&result, iced_x86::Code::And_rm64_r64);
    }

    #[test]
    fn test_or_memory_imm8() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("or [rbx], 0x1").unwrap();
        assert_ice(&result, iced_x86::Code::Or_rm64_imm8);
    }

    #[test]
    fn test_xor_memory_reg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("xor [rcx], rdx").unwrap();
        assert_ice(&result, iced_x86::Code::Xor_rm64_r64);
    }

    #[test]
    fn test_not_memory() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("not [rdx]").unwrap();
        assert_ice(&result, iced_x86::Code::Not_rm64);
    }

    #[test]
    fn test_neg_memory() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("neg [rsi]").unwrap();
        assert_ice(&result, iced_x86::Code::Neg_rm64);
    }
}