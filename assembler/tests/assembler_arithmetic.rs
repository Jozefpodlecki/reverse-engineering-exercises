mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_asm_add_reg_reg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("add rax, rbx").unwrap();
        assert_eq!(result, vec![0x48, 0x01, 0xD8]);
        assert_ice(&result, iced_x86::Code::Add_rm64_r64);
    }

    #[test]
    fn test_asm_add_reg_imm() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("add rax, 0x10").unwrap();
        assert_ice(&result, iced_x86::Code::Add_rm64_imm8);
    }

    #[test]
    fn test_asm_sub_reg_reg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("sub rax, rbx").unwrap();
        assert_eq!(result, vec![0x48, 0x29, 0xD8]);
        assert_ice(&result, iced_x86::Code::Sub_rm64_r64);
    }

    #[test]
    fn test_asm_sub_rsp_imm() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("sub rsp, 0x38").unwrap();
        assert_eq!(result, vec![0x48, 0x83, 0xEC, 0x38]);
        assert_ice(&result, iced_x86::Code::Sub_rm64_imm8);
    }

    #[test]
    fn test_asm_xor_reg_reg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("xor rax, rax").unwrap();
        assert_eq!(result, vec![0x48, 0x31, 0xC0]);
        assert_ice(&result, iced_x86::Code::Xor_rm64_r64);
    }

    #[test]
    fn test_asm_and_reg_reg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("and rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::And_rm64_r64);
    }

    #[test]
    fn test_asm_or_reg_reg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("or rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Or_rm64_r64);
    }

    #[test]
    fn test_asm_inc() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("inc rax").unwrap();
        assert_ice(&result, iced_x86::Code::Inc_rm64);
    }

    #[test]
    fn test_asm_dec() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("dec rax").unwrap();
        assert_ice(&result, iced_x86::Code::Dec_rm64);
    }

    #[test]
    fn test_asm_neg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("neg rax").unwrap();
        assert_ice(&result, iced_x86::Code::Neg_rm64);
    }

    #[test]
    fn test_asm_not() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("not rax").unwrap();
        assert_ice(&result, iced_x86::Code::Not_rm64);
    }
}