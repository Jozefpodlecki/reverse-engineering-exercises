mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_asm_syscall() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("syscall").unwrap();
        assert_eq!(result, vec![0x0F, 0x05]);
        assert_ice(&result, iced_x86::Code::Syscall);
    }

    #[test]
    fn test_asm_sysenter() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("sysenter").unwrap();
        assert_eq!(result, vec![0x0F, 0x34]);
        assert_ice(&result, iced_x86::Code::Sysenter);
    }

    #[test]
    fn test_asm_sysexit() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("sysexit").unwrap();
        assert_eq!(result, vec![0x0F, 0x35]);
        assert_ice(&result, iced_x86::Code::Sysexitd);
    }

    #[test]
    fn test_asm_cpuid() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("cpuid").unwrap();
        assert_eq!(result, vec![0x0F, 0xA2]);
        assert_ice(&result, iced_x86::Code::Cpuid);
    }

    #[test]
    fn test_asm_rdtsc() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("rdtsc").unwrap();
        assert_eq!(result, vec![0x0F, 0x31]);
        assert_ice(&result, iced_x86::Code::Rdtsc);
    }

    #[test]
    fn test_asm_ret() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("ret").unwrap();
        assert_eq!(result, vec![0xC3]);
        assert_ice(&result, iced_x86::Code::Retnq);
    }

    #[test]
    fn test_asm_nop() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("nop").unwrap();
        assert_eq!(result, vec![0x90]);
        assert_ice(&result, iced_x86::Code::Nopd);
    }

    #[test]
    fn test_asm_hlt() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("hlt").unwrap();
        assert_eq!(result, vec![0xF4]);
        assert_ice(&result, iced_x86::Code::Hlt);
    }

    #[test]
    fn test_asm_int3() {
        let mut assembler = AssemblerNoSymbols::new();
        let result = assembler.assemble_str("int3").unwrap();
        assert_eq!(result, vec![0xCC]);
        assert_ice(&result, iced_x86::Code::Int3);
    }
}