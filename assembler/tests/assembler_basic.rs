
#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;

    use super::*;

    #[test]
    fn test_asm_syscall() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("syscall").unwrap();
        assert_eq!(result, vec![15, 5]);

        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();

        assert_eq!(instr.code(), iced_x86::Code::Syscall)
    }

    #[test]
    fn test_complex() {
        let assembler = Assembler::new();
        let source = LineSource::from(["sub rsp, 0x38", "mov [rsp + 0x30], r15"]);
        let result = assembler.assemble(source).unwrap();
        assert_eq!(result, vec![72, 131, 236, 56, 76, 137, 124, 36, 48]);

        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Sub_rm64_imm8);

        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Mov_rm64_r64)
    }
}