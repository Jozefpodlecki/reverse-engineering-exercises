
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    // 64-bit
    RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI,
    R8, R9, R10, R11, R12, R13, R14, R15,
    
    // 32-bit
    EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI,
    
    // 16-bit
    AX, CX, DX, BX, SP, BP, SI, DI,
    
    // 8-bit
    AL, CL, DL, BL, AH, CH, DH, BH,
    SPL, BPL, SIL, DIL,
    R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B,
    
    // SIMD
    XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7,
    XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15,
    
    YMM0, YMM1, YMM2, YMM3, YMM4, YMM5, YMM6, YMM7,
    YMM8, YMM9, YMM10, YMM11, YMM12, YMM13, YMM14, YMM15,
    
    ZMM0, ZMM1, ZMM2, ZMM3, ZMM4, ZMM5, ZMM6, ZMM7,
    ZMM8, ZMM9, ZMM10, ZMM11, ZMM12, ZMM13, ZMM14, ZMM15,
    ZMM16, ZMM17, ZMM18, ZMM19, ZMM20, ZMM21, ZMM22, ZMM23,
    ZMM24, ZMM25, ZMM26, ZMM27, ZMM28, ZMM29, ZMM30, ZMM31,
    
    // Segment
    ES, CS, SS, DS, FS, GS,
    
    // Control
    CR0, CR1, CR2, CR3, CR4, CR5, CR6, CR7, CR8,
    
    // Debug
    DR0, DR1, DR2, DR3, DR4, DR5, DR6, DR7,
    
    // MMX
    MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7,
    
    // Mask (AVX-512)
    K0, K1, K2, K3, K4, K5, K6, K7,
}

impl Register {
    pub fn from_str(s: &str) -> Option<Self> {
        use Register::*;
        match s {
            "rax" => Some(RAX), "rcx" => Some(RCX), "rdx" => Some(RDX), "rbx" => Some(RBX),
            "rsp" => Some(RSP), "rbp" => Some(RBP), "rsi" => Some(RSI), "rdi" => Some(RDI),
            "r8" => Some(R8), "r9" => Some(R9), "r10" => Some(R10), "r11" => Some(R11),
            "r12" => Some(R12), "r13" => Some(R13), "r14" => Some(R14), "r15" => Some(R15),
            "eax" => Some(EAX), "ecx" => Some(ECX), "edx" => Some(EDX), "ebx" => Some(EBX),
            "esp" => Some(ESP), "ebp" => Some(EBP), "esi" => Some(ESI), "edi" => Some(EDI),
            "ax" => Some(AX), "cx" => Some(CX), "dx" => Some(DX), "bx" => Some(BX),
            "sp" => Some(SP), "bp" => Some(BP), "si" => Some(SI), "di" => Some(DI),
            "al" => Some(AL), "cl" => Some(CL), "dl" => Some(DL), "bl" => Some(BL),
            "ah" => Some(AH), "ch" => Some(CH), "dh" => Some(DH), "bh" => Some(BH),
            "spl" => Some(SPL), "bpl" => Some(BPL), "sil" => Some(SIL), "dil" => Some(DIL),
            "r8b" => Some(R8B), "r9b" => Some(R9B), "r10b" => Some(R10B), "r11b" => Some(R11B),
            "r12b" => Some(R12B), "r13b" => Some(R13B), "r14b" => Some(R14B), "r15b" => Some(R15B),
            "xmm0" => Some(XMM0), "xmm1" => Some(XMM1), "xmm2" => Some(XMM2), "xmm3" => Some(XMM3),
            "xmm4" => Some(XMM4), "xmm5" => Some(XMM5), "xmm6" => Some(XMM6), "xmm7" => Some(XMM7),
            "xmm8" => Some(XMM8), "xmm9" => Some(XMM9), "xmm10" => Some(XMM10), "xmm11" => Some(XMM11),
            "xmm12" => Some(XMM12), "xmm13" => Some(XMM13), "xmm14" => Some(XMM14), "xmm15" => Some(XMM15),
            "ymm0" => Some(YMM0), "ymm1" => Some(YMM1), "ymm2" => Some(YMM2), "ymm3" => Some(YMM3),
            "ymm4" => Some(YMM4), "ymm5" => Some(YMM5), "ymm6" => Some(YMM6), "ymm7" => Some(YMM7),
            "ymm8" => Some(YMM8), "ymm9" => Some(YMM9), "ymm10" => Some(YMM10), "ymm11" => Some(YMM11),
            "ymm12" => Some(YMM12), "ymm13" => Some(YMM13), "ymm14" => Some(YMM14), "ymm15" => Some(YMM15),
            "zmm0" => Some(ZMM0), "zmm1" => Some(ZMM1), "zmm2" => Some(ZMM2), "zmm3" => Some(ZMM3),
            "zmm4" => Some(ZMM4), "zmm5" => Some(ZMM5), "zmm6" => Some(ZMM6), "zmm7" => Some(ZMM7),
            "zmm8" => Some(ZMM8), "zmm9" => Some(ZMM9), "zmm10" => Some(ZMM10), "zmm11" => Some(ZMM11),
            "zmm12" => Some(ZMM12), "zmm13" => Some(ZMM13), "zmm14" => Some(ZMM14), "zmm15" => Some(ZMM15),
            "zmm16" => Some(ZMM16), "zmm17" => Some(ZMM17), "zmm18" => Some(ZMM18), "zmm19" => Some(ZMM19),
            "zmm20" => Some(ZMM20), "zmm21" => Some(ZMM21), "zmm22" => Some(ZMM22), "zmm23" => Some(ZMM23),
            "zmm24" => Some(ZMM24), "zmm25" => Some(ZMM25), "zmm26" => Some(ZMM26), "zmm27" => Some(ZMM27),
            "zmm28" => Some(ZMM28), "zmm29" => Some(ZMM29), "zmm30" => Some(ZMM30), "zmm31" => Some(ZMM31),
            "es" => Some(ES), "cs" => Some(CS), "ss" => Some(SS), "ds" => Some(DS),
            "fs" => Some(FS), "gs" => Some(GS),
            "cr0" => Some(CR0), "cr1" => Some(CR1), "cr2" => Some(CR2), "cr3" => Some(CR3),
            "cr4" => Some(CR4), "cr5" => Some(CR5), "cr6" => Some(CR6), "cr7" => Some(CR7), "cr8" => Some(CR8),
            "dr0" => Some(DR0), "dr1" => Some(DR1), "dr2" => Some(DR2), "dr3" => Some(DR3),
            "dr4" => Some(DR4), "dr5" => Some(DR5), "dr6" => Some(DR6), "dr7" => Some(DR7),
            "mm0" => Some(MM0), "mm1" => Some(MM1), "mm2" => Some(MM2), "mm3" => Some(MM3),
            "mm4" => Some(MM4), "mm5" => Some(MM5), "mm6" => Some(MM6), "mm7" => Some(MM7),
            "k0" => Some(K0), "k1" => Some(K1), "k2" => Some(K2), "k3" => Some(K3),
            "k4" => Some(K4), "k5" => Some(K5), "k6" => Some(K6), "k7" => Some(K7),
            _ => None,
        }
    }
    
    pub fn code(&self) -> u8 {
        use Register::*;
        match self {
            RAX | EAX | AX | AL => 0,
            RCX | ECX | CX | CL => 1,
            RDX | EDX | DX | DL => 2,
            RBX | EBX | BX | BL => 3,
            RSP | ESP | SP | SPL => 4,
            RBP | EBP | BP | BPL => 5,
            RSI | ESI | SI | SIL => 6,
            RDI | EDI | DI | DIL => 7,
            R8 | R8B => 8, R9 | R9B => 9, R10 | R10B => 10, R11 | R11B => 11,
            R12 | R12B => 12, R13 | R13B => 13, R14 | R14B => 14, R15 | R15B => 15,
            _ => 0,
        }
    }
    
    pub fn is_extended(&self) -> bool {
        matches!(self, Self::R8 | Self::R9 | Self::R10 | Self::R11 | Self::R12 | Self::R13 | Self::R14 | Self::R15)
    }
}

impl Register {
    pub fn is_xmm(&self) -> bool {
        matches!(self, 
            Register::XMM0 | Register::XMM1 | Register::XMM2 | Register::XMM3 |
            Register::XMM4 | Register::XMM5 | Register::XMM6 | Register::XMM7 |
            Register::XMM8 | Register::XMM9 | Register::XMM10 | Register::XMM11 |
            Register::XMM12 | Register::XMM13 | Register::XMM14 | Register::XMM15
        )
    }

    pub fn is_ymm(&self) -> bool {
        matches!(self,
            Register::YMM0 | Register::YMM1 | Register::YMM2 | Register::YMM3 |
            Register::YMM4 | Register::YMM5 | Register::YMM6 | Register::YMM7 |
            Register::YMM8 | Register::YMM9 | Register::YMM10 | Register::YMM11 |
            Register::YMM12 | Register::YMM13 | Register::YMM14 | Register::YMM15
        )
    }

    pub fn is_zmm(&self) -> bool {
        matches!(self,
            Register::ZMM0 | Register::ZMM1 | Register::ZMM2 | Register::ZMM3 |
            Register::ZMM4 | Register::ZMM5 | Register::ZMM6 | Register::ZMM7 |
            Register::ZMM8 | Register::ZMM9 | Register::ZMM10 | Register::ZMM11 |
            Register::ZMM12 | Register::ZMM13 | Register::ZMM14 | Register::ZMM15 |
            Register::ZMM16 | Register::ZMM17 | Register::ZMM18 | Register::ZMM19 |
            Register::ZMM20 | Register::ZMM21 | Register::ZMM22 | Register::ZMM23 |
            Register::ZMM24 | Register::ZMM25 | Register::ZMM26 | Register::ZMM27 |
            Register::ZMM28 | Register::ZMM29 | Register::ZMM30 | Register::ZMM31
        )
    }

    pub fn is_gp(&self) -> bool {
        matches!(self,
            Register::RAX | Register::RCX | Register::RDX | Register::RBX |
            Register::RSP | Register::RBP | Register::RSI | Register::RDI |
            Register::R8 | Register::R9 | Register::R10 | Register::R11 |
            Register::R12 | Register::R13 | Register::R14 | Register::R15
        )
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::RAX => "rax", Self::RCX => "rcx", Self::RDX => "rdx", Self::RBX => "rbx",
            Self::RSP => "rsp", Self::RBP => "rbp", Self::RSI => "rsi", Self::RDI => "rdi",
            Self::R8 => "r8", Self::R9 => "r9", Self::R10 => "r10", Self::R11 => "r11",
            Self::R12 => "r12", Self::R13 => "r13", Self::R14 => "r14", Self::R15 => "r15",
            Self::EAX => "eax", Self::ECX => "ecx", Self::EDX => "edx", Self::EBX => "ebx",
            Self::ESP => "esp", Self::EBP => "ebp", Self::ESI => "esi", Self::EDI => "edi",
            Self::AX => "ax", Self::CX => "cx", Self::DX => "dx", Self::BX => "bx",
            Self::SP => "sp", Self::BP => "bp", Self::SI => "si", Self::DI => "di",
            Self::AL => "al", Self::CL => "cl", Self::DL => "dl", Self::BL => "bl",
            Self::AH => "ah", Self::CH => "ch", Self::DH => "dh", Self::BH => "bh",
            Self::SPL => "spl", Self::BPL => "bpl", Self::SIL => "sil", Self::DIL => "dil",
            Self::R8B => "r8b", Self::R9B => "r9b", Self::R10B => "r10b", Self::R11B => "r11b",
            Self::R12B => "r12b", Self::R13B => "r13b", Self::R14B => "r14b", Self::R15B => "r15b",
            Self::XMM0 => "xmm0", Self::XMM1 => "xmm1", Self::XMM2 => "xmm2", Self::XMM3 => "xmm3",
            Self::XMM4 => "xmm4", Self::XMM5 => "xmm5", Self::XMM6 => "xmm6", Self::XMM7 => "xmm7",
            Self::XMM8 => "xmm8", Self::XMM9 => "xmm9", Self::XMM10 => "xmm10", Self::XMM11 => "xmm11",
            Self::XMM12 => "xmm12", Self::XMM13 => "xmm13", Self::XMM14 => "xmm14", Self::XMM15 => "xmm15",
            Self::YMM0 => "ymm0", Self::YMM1 => "ymm1", Self::YMM2 => "ymm2", Self::YMM3 => "ymm3",
            Self::YMM4 => "ymm4", Self::YMM5 => "ymm5", Self::YMM6 => "ymm6", Self::YMM7 => "ymm7",
            Self::YMM8 => "ymm8", Self::YMM9 => "ymm9", Self::YMM10 => "ymm10", Self::YMM11 => "ymm11",
            Self::YMM12 => "ymm12", Self::YMM13 => "ymm13", Self::YMM14 => "ymm14", Self::YMM15 => "ymm15",
            Self::ZMM0 => "zmm0", Self::ZMM1 => "zmm1", Self::ZMM2 => "zmm2", Self::ZMM3 => "zmm3",
            Self::ZMM4 => "zmm4", Self::ZMM5 => "zmm5", Self::ZMM6 => "zmm6", Self::ZMM7 => "zmm7",
            Self::ZMM8 => "zmm8", Self::ZMM9 => "zmm9", Self::ZMM10 => "zmm10", Self::ZMM11 => "zmm11",
            Self::ZMM12 => "zmm12", Self::ZMM13 => "zmm13", Self::ZMM14 => "zmm14", Self::ZMM15 => "zmm15",
            Self::ZMM16 => "zmm16", Self::ZMM17 => "zmm17", Self::ZMM18 => "zmm18", Self::ZMM19 => "zmm19",
            Self::ZMM20 => "zmm20", Self::ZMM21 => "zmm21", Self::ZMM22 => "zmm22", Self::ZMM23 => "zmm23",
            Self::ZMM24 => "zmm24", Self::ZMM25 => "zmm25", Self::ZMM26 => "zmm26", Self::ZMM27 => "zmm27",
            Self::ZMM28 => "zmm28", Self::ZMM29 => "zmm29", Self::ZMM30 => "zmm30", Self::ZMM31 => "zmm31",
            Self::ES => "es", Self::CS => "cs", Self::SS => "ss", Self::DS => "ds",
            Self::FS => "fs", Self::GS => "gs",
            Self::CR0 => "cr0", Self::CR1 => "cr1", Self::CR2 => "cr2", Self::CR3 => "cr3",
            Self::CR4 => "cr4", Self::CR5 => "cr5", Self::CR6 => "cr6", Self::CR7 => "cr7", Self::CR8 => "cr8",
            Self::DR0 => "dr0", Self::DR1 => "dr1", Self::DR2 => "dr2", Self::DR3 => "dr3",
            Self::DR4 => "dr4", Self::DR5 => "dr5", Self::DR6 => "dr6", Self::DR7 => "dr7",
            Self::MM0 => "mm0", Self::MM1 => "mm1", Self::MM2 => "mm2", Self::MM3 => "mm3",
            Self::MM4 => "mm4", Self::MM5 => "mm5", Self::MM6 => "mm6", Self::MM7 => "mm7",
            Self::K0 => "k0", Self::K1 => "k1", Self::K2 => "k2", Self::K3 => "k3",
            Self::K4 => "k4", Self::K5 => "k5", Self::K6 => "k6", Self::K7 => "k7",
        };
        write!(f, "{}", s)
    }
}