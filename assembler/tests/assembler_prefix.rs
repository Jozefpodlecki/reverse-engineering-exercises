mod helpers;

#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;
    use helpers::assert_ice;
    use super::*;

    #[test]
    fn test_lock_inc() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("lock inc [rax]").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Inc_rm64);
        assert!(instr.has_lock_prefix());
    }

    #[test]
    fn test_lock_add() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("lock add [rbx], 1").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Add_rm64_imm8);
        assert!(instr.has_lock_prefix());
    }

    #[test]
    fn test_lock_xchg() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("lock xchg [rcx], rax").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Xchg_rm64_r64);
        assert!(instr.has_lock_prefix());
    }

    #[test]
    fn test_rep_movsb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("rep movsb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Movsb_m8_m8);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_stosb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("rep stosb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosb_m8_AL);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_stosd() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("rep stosd").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosd_m32_EAX);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_cmpsb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("rep cmpsb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Cmpsb_m8_m8);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_rep_scasb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("rep scasb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Scasb_AL_m8);
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_repne_scasb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("repne scasb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Scasb_AL_m8);
        assert!(instr.has_repne_prefix());
    }

    #[test]
    fn test_repne_cmpsb() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("repne cmpsb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Cmpsb_m8_m8);
        assert!(instr.has_repne_prefix());
    }

    #[test]
    fn test_multiple_prefixes() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("lock rep stosb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosb_m8_AL);
        assert!(instr.has_lock_prefix());
        assert!(instr.has_rep_prefix());
    }

    #[test]
    fn test_no_prefix() {
        let assembler = Assembler::new();
        let result = assembler.assemble_str("stosb").unwrap();
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        assert_eq!(instr.code(), iced_x86::Code::Stosb_m8_AL);
        assert!(!instr.has_lock_prefix());
        assert!(!instr.has_rep_prefix());
    }
}