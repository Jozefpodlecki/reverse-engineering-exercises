mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_asm_cmove() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmove rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmove_r64_rm64);
    }

    #[test]
    fn test_asm_cmovne() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovne rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovne_r64_rm64);
    }

    #[test]
    fn test_asm_cmovg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovg rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovg_r64_rm64);
    }

    #[test]
    fn test_asm_cmovge() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovge rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovge_r64_rm64);
    }

    #[test]
    fn test_asm_cmovl() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovl rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovl_r64_rm64);
    }

    #[test]
    fn test_asm_cmovle() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovle rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovle_r64_rm64);
    }

    #[test]
    fn test_asm_cmova() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmova rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmova_r64_rm64);
    }

    #[test]
    fn test_asm_cmovae() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovae rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovae_r64_rm64);
    }

    #[test]
    fn test_asm_cmovb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovb rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovb_r64_rm64);
    }

    #[test]
    fn test_asm_cmovbe() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovbe rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovbe_r64_rm64);
    }

    #[test]
    fn test_asm_cmovs() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovs rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovs_r64_rm64);
    }

    #[test]
    fn test_asm_cmovns() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovns rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovns_r64_rm64);
    }

    #[test]
    fn test_asm_cmovz() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovz rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmove_r64_rm64);
    }

    #[test]
    fn test_asm_cmovnz() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("cmovnz rax, rbx").unwrap();
        assert_ice(&result, iced_x86::Code::Cmovne_r64_rm64);
    }
}