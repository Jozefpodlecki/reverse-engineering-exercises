mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_push_rax() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rax").unwrap();
        assert_eq!(result, vec![0x50]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rcx() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rcx").unwrap();
        assert_eq!(result, vec![0x51]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rdx() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rdx").unwrap();
        assert_eq!(result, vec![0x52]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rbx() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rbx").unwrap();
        assert_eq!(result, vec![0x53]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rsp() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rsp").unwrap();
        assert_eq!(result, vec![0x54]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rbp() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rbp").unwrap();
        assert_eq!(result, vec![0x55]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rsi() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rsi").unwrap();
        assert_eq!(result, vec![0x56]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_rdi() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push rdi").unwrap();
        assert_eq!(result, vec![0x57]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_r8() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push r8").unwrap();
        assert_eq!(result, vec![0x41, 0x50]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_r15() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push r15").unwrap();
        assert_eq!(result, vec![0x41, 0x57]);
        assert_ice(&result, iced_x86::Code::Push_r64);
    }

    #[test]
    fn test_push_imm8() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("push 0x38").unwrap();
        assert_eq!(result, vec![0x68, 0x38, 0x00, 0x00, 0x00]);
        assert_ice(&result, iced_x86::Code::Pushq_imm32);
    }

    #[test]
    fn test_pop_rax() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("pop rax").unwrap();
        assert_eq!(result, vec![0x58]);
        assert_ice(&result, iced_x86::Code::Pop_r64);
    }

    #[test]
    fn test_pop_rcx() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("pop rcx").unwrap();
        assert_eq!(result, vec![0x59]);
        assert_ice(&result, iced_x86::Code::Pop_r64);
    }

    #[test]
    fn test_pop_r8() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("pop r8").unwrap();
        assert_eq!(result, vec![0x41, 0x58]);
        assert_ice(&result, iced_x86::Code::Pop_r64);
    }

    #[test]
    fn test_push_pop_sequence() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "push rax\npush rcx\npop rcx\npop rax";
        let result = assembler.assemble_str(source).unwrap();
        assert_eq!(result, vec![0x50, 0x51, 0x59, 0x58]);
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        let instr3 = decoder.decode();
        let instr4 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Push_r64);
        assert_eq!(instr2.code(), iced_x86::Code::Push_r64);
        assert_eq!(instr3.code(), iced_x86::Code::Pop_r64);
        assert_eq!(instr4.code(), iced_x86::Code::Pop_r64);
    }

    #[test]
    fn test_enter() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("enter 0x10, 0x0").unwrap();
        assert_eq!(result, vec![0xC8, 0x10, 0x00, 0x00]);
        assert_ice(&result, iced_x86::Code::Enterq_imm16_imm8);
    }

    #[test]
    fn test_enter_with_nest_level() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("enter 0x20, 0x3").unwrap();
        assert_eq!(result, vec![0xC8, 0x20, 0x00, 0x03]);
        assert_ice(&result, iced_x86::Code::Enterq_imm16_imm8);
    }

    #[test]
    fn test_leave() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("leave").unwrap();
        assert_eq!(result, vec![0xC9]);
        assert_ice(&result, iced_x86::Code::Leaveq);
    }
}