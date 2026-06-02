use std::rc::Rc;


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
    pub fn create(kind: &str, bitness: u32) -> Rc<dyn Decoder> {
        match kind {
            "iced-x86" => Rc::new(IcedDecoder::new(bitness)),
            "prometheus" => Rc::new(PrometheusDecoder::new()),
            _ => unreachable!("Unknown decoder")
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