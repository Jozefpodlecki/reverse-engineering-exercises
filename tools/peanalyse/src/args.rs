use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    #[arg(long, short, required = true)]
    pub input: String,

    /// Show summary (optional flag)
    #[arg(long)]
    pub summary: bool,

    /// Shows image base and entrypoint
    #[arg(long)]
    pub entry: bool,

    /// 
    #[arg(long)]
    pub disassemble_text: bool,

    /// Shows iat entries
    #[arg(long)]
    pub iat_entries: bool,

     /// Prints value at given addr
    #[arg(
        long,
        value_parser = parse_hex_or_decimal
    )]
    pub read_addr: Option<u64>,
 
    /// Virtual address to start disassembly (hex 0x... or decimal)
    #[arg(
        long,
        value_parser = parse_hex_or_decimal
    )]
    pub disassemble_from_addr: Option<u64>,

    /// Virtual address to stop disassembly (hex 0x... or decimal)
    #[arg(long, value_parser = parse_hex_or_decimal)]
    pub disassemble_to_addr: Option<u64>,

    /// Stop disassembly when INT3 (0xCC) instruction is encountered
    #[arg(long, default_value_t = true)]
    pub stop_on_int3: bool,
}

fn parse_hex_or_decimal(s: &str) -> Result<u64, String> {
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else {
        s.parse::<u64>().map_err(|e| e.to_string())
    }
}