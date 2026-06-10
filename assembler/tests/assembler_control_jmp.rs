#[cfg(test)]
pub mod tests {
    use assembler::*;
    use iced_x86::DecoderOptions;
    use super::*;

    #[test]
    fn test_jmp_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jmp target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jmp_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    // #[test]
    // fn test_jmp_forward() {
    //     let mut assembler = AssemblerNoSymbols::new();
    //     let source = "jmp target\nmov rax, 1\nmov rbx, 2\ntarget:\nret";
    //     let (result, _) = assembler.assemble_with_symbols(source).unwrap();
        
    //     let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
    //     let instr1 = decoder.decode();
        
    //     assert_eq!(instr1.code(), iced_x86::Code::Jmp_rel32_64);
        
    //     let jmp_offset = i32::from_le_bytes([result[1], result[2], result[3], result[4]]);
    //     assert_eq!(jmp_offset, 20);
    // }

    #[test]
    fn test_jmp_backward() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "target:\nret\njmp target";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Retnq);
        assert_eq!(instr2.code(), iced_x86::Code::Jmp_rel32_64);
        
        let jmp_offset = i32::from_le_bytes([result[3], result[4], result[5], result[6]]);
        assert_eq!(jmp_offset, -5);
    }

    #[test]
    fn test_je_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "je target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Je_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jne_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jne target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jne_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jg_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jg target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jg_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jge_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jge target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jge_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jl_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jl target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jl_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jle_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jle target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jle_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_ja_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "ja target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Ja_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jb_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jb target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jb_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jz_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jz target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Je_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_jnz_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jnz target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Je_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_call_label() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "call target\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Call_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Retnq);
    }

    #[test]
    fn test_multiple_labels() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "jmp start\nmov rax, 1\nstart:\njmp end\nmov rbx, 2\nend:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        let instr3 = decoder.decode();
        let instr4 = decoder.decode();
        let instr5 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Jmp_rel32_64);
        assert_eq!(instr2.code(), iced_x86::Code::Mov_r64_imm64);
        assert_eq!(instr3.code(), iced_x86::Code::Jmp_rel32_64);
        assert_eq!(instr4.code(), iced_x86::Code::Mov_r64_imm64);
        assert_eq!(instr5.code(), iced_x86::Code::Retnq);
    }

   #[test]
    fn test_label_resolution_with_data() {
        let mut assembler = AssemblerNoSymbols::new();
        let source = "mov rax, 0\njmp target\nmov rax, 1\ntarget:\nret";
        let result = assembler.assemble_str(source).unwrap();
        
        let mut decoder = iced_x86::Decoder::new(64, &result, DecoderOptions::NONE);
        let instr1 = decoder.decode();
        let instr2 = decoder.decode();
        let instr3 = decoder.decode();
        let instr4 = decoder.decode();
        
        assert_eq!(instr1.code(), iced_x86::Code::Mov_r64_imm64);
        assert_eq!(instr2.code(), iced_x86::Code::Jmp_rel32_64);
        assert_eq!(instr3.code(), iced_x86::Code::Mov_r64_imm64);
        assert_eq!(instr4.code(), iced_x86::Code::Retnq);
    }
}