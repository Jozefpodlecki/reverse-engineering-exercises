#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;

    use super::*;

    #[test]
    fn test_asm_simd_movsd() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("movsd xmm0, xmm1").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::Movsd_xmm_xmmm64);
    }

    #[test]
    fn test_asm_simd_movss() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("movss xmm0, [rax]").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::Movss_xmm_xmmm32);
    }

    #[test]
    fn test_asm_simd_addpd() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("addpd xmm0, xmm1").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::Addpd_xmm_xmmm128);
    }

    #[test]
    fn test_asm_simd_mulps() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("mulps xmm0, xmm1").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::Mulps_xmm_xmmm128);
    }

    #[test]
    fn test_asm_avx_vaddpd() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("vaddpd xmm0, xmm1, xmm2").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::VEX_Vaddpd_xmm_xmm_xmmm128);
    }

    #[test]
    fn test_asm_avx2_vmovdqa() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("vmovdqa ymm0, ymm1").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::VEX_Vmovdqa_ymm_ymmm256);
    }

    #[test]
    fn test_asm_avx512_vaddpd_zmm() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("vaddpd zmm0, zmm1, zmm2").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::EVEX_Vaddpd_zmm_k1z_zmm_zmmm512b64_er);
    }

    #[test]
    fn test_asm_avx512_vmovdqa32() {
        let mut assembler = Assembler::new();
        let result = assembler.assemble_str("vmovdqa32 zmm0, zmm1").unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr = decoder.decode();
        
        assert_eq!(instr.code(), iced_x86::Code::EVEX_Vmovdqa32_xmm_k1z_xmmm128);
    }
}