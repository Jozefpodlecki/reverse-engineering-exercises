#[derive(Debug, Clone, PartialEq)]
pub enum DecoderKind {
    IcedX86,
    Prometheus,
}

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: String,
    pub name: String,
    pub decoder_type: DecoderKind,
    pub instructions: Vec<AsmInstruction>,
    pub registers: Registers
}

#[derive(Default, Clone, PartialEq)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[derive(Default, Clone, PartialEq)]
pub struct AsmInstruction {
    pub asm: String,
    pub bytes: Vec<u8>
}

pub trait Decoder: Send + Sync {
    fn decode(&self, bytes: &[u8], ip: u64) -> Result<DecodedInstruction, String>;
    fn name(&self) -> &'static str;
}

pub struct DecodedInstruction {
    pub bytes: Vec<u8>,
    pub asm: String,
    pub mnemonic: String,
    pub operands: String,
    pub size: u8,
}

pub struct DecoderFactory;

impl DecoderFactory {
    pub fn create(kind: &DecoderKind, bitness: u32) -> Box<dyn Decoder> {
        match kind {
            DecoderKind::IcedX86 => Box::new(IcedDecoder::new(bitness)),
            DecoderKind::Prometheus => Box::new(PrometheusDecoder::new()),
        }
    }
}

pub struct IcedDecoder {
    bitness: u32,
}

impl IcedDecoder {
    pub fn new(bitness: u32) -> Self {
        Self { bitness }
    }
}

impl Decoder for IcedDecoder {
    fn decode(&self, bytes: &[u8], ip: u64) -> Result<DecodedInstruction, String> {
        let mut decoder = iced_x86::Decoder::with_ip(self.bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
        let instruction = decoder.decode();
        
        if instruction.is_invalid() {
            log::info!("invalid {:?}", bytes);
            return Err("Invalid instruction".to_string());
        }
        
        let mut formatter = iced_x86::NasmFormatter::new();
        let mut output = String::new();
        iced_x86::Formatter::format(&mut formatter, &instruction, &mut output);
        
        let parts: Vec<&str> = output.splitn(2, ' ').collect();
        let mnemonic = parts[0].to_string();
        let operands = parts.get(1).unwrap_or(&"").to_string();
        
        Ok(DecodedInstruction {
            bytes: bytes[..instruction.len()].to_vec(),
            asm: output,
            mnemonic,
            operands,
            size: instruction.len() as u8,
        })
    }
    
    fn name(&self) -> &'static str {
        "iced-x86"
    }
}

pub struct PrometheusDecoder {
    inner: prometheus_disassembler::Decoder,
}

impl PrometheusDecoder {
    pub fn new() -> Self {
        Self {
            inner: prometheus_disassembler::Decoder::new(prometheus_disassembler::Architecture::X64),
        }
    }
}

impl Decoder for PrometheusDecoder {
    fn decode(&self, bytes: &[u8], ip: u64) -> Result<DecodedInstruction, String> {
        let instruction = self.inner.decode(bytes, ip).map_err(|e| e.to_string())?;
        let formatter = prometheus_disassembler::Formatter::new(prometheus_disassembler::formatter::Syntax::Intel);

        Ok(DecodedInstruction {
            bytes: instruction.bytes.to_vec(),
            asm: formatter.format(&instruction),
            mnemonic: format!("{:?}", instruction.mnemonic),
            operands: format!("{:?}", instruction.operands),
            size: instruction.bytes.len() as u8
        })
    }
    
    fn name(&self) -> &'static str {
        "prometheus"
    }
}