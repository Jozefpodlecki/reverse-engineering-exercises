mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;
    use helpers::assert_ice;
    use super::*;

    // #[test]
    // fn test_multiple_prefixes() {
    //     let mut assembler = Assembler::new();
    //     let result = assembler.assemble_str("lock rep stosb\nret").unwrap();
        
    //     let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        
    //     let instr1 = decoder.decode();  // stosb - decodes successfully
    //     let instr2 = decoder.decode();  // ret
        
    //     assert_eq!(instr1.code(), iced_x86::Code::Stosb_m8_AL);
    //     assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    //     assert!(instr1.has_lock_prefix());
    //     assert!(instr1.has_rep_prefix());
    // }

    #[test]
    fn test_lock_inc() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("lock inc [rax]").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Inc_rm64);
        assert!(instr.has_lock_prefix());
    }

    #[test]
    fn test_lock_add() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("lock add [rbx], 1").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Add_rm64_imm8);
        assert!(instr.has_lock_prefix());
    }

    #[test]
    fn test_lock_xchg() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("lock xchg [rcx], rax").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Xchg_rm64_r64);
        assert!(instr.has_lock_prefix());
    }

    #[test]
    fn test_rep_movsb() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("rep movsb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Movsb_m8_m8);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_stosb() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("rep stosb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosb_m8_AL);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_stosd() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("rep stosd").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosd_m32_EAX);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_cmpsb() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("rep cmpsb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Cmpsb_m8_m8);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_scasb() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("rep scasb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Scasb_AL_m8);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_repne_scasb() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("repne scasb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Scasb_AL_m8);
        assert!(instr.has_repne_prefix());
    }

    #[test]
    fn test_repne_cmpsb() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("repne cmpsb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Cmpsb_m8_m8);
        assert!(instr.has_repne_prefix());
    }

    #[test]
    fn test_no_prefix() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("stosb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosb_m8_AL);
        assert!(!instr.has_lock_prefix());
        assert!(!instr.has_rep_prefix());
    }
}