use anyhow::Result;
use capstone::{arch::{self, x86::{X86Insn, X86Reg}, BuildsCapstone, BuildsCapstoneSyntax, DetailsArchInsn}, Capstone, Insn};
use pelite::{pe::Pe, PeFile, Wrap};

pub fn build_capstone() -> Result<Capstone> {
    let mut cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .build()?;
    cs.set_skipdata(true)?;
    cs.set_detail(true)?;
    Ok(cs)
}

pub fn print_imports(file: PeFile<'_>) -> Result<()> {
    match file {
        Wrap::T32(pe) => {
            let imports = pelite::pe32::Pe::imports(pe)?;
            let optional_header = pelite::pe32::Pe::optional_header(pe);
            let image_base = optional_header.ImageBase as u64;

            for desc in imports.iter() {
                println!("DLL: {}", desc.dll_name()?);
                let iat = desc.iat()?;
                let int = desc.int()?;
                for (rva, import) in iat.zip(int) {
                    let va = image_base + *rva as u64;

                    match import? {
                        pelite::pe::imports::Import::ByName { hint, name } => {
                            println!("  {} @ RVA 0x{:08X} VA 0x{:08X} (hint {})", 
                                     name.to_str()?, rva, va, hint)
                        }
                        pelite::pe::imports::Import::ByOrdinal { ord } => {
                            println!("  Ordinal {} @ RVA 0x{:08X} VA 0x{:08X}", ord, rva, va)
                        }
                    }
                }
            }
        }
        Wrap::T64(pe) => {
            let imports = pelite::pe64::PeFile::imports(pe)?;
            let optional_header = pe.optional_header();
            let image_base = optional_header.ImageBase;

            for desc in imports.iter() {
                println!("DLL: {}", desc.dll_name()?);
                let iat = desc.iat()?;
                let int = desc.int()?;
                for (rva, import) in iat.zip(int) {
                    let va = image_base + *rva as u64;
                    match import? {
                        pelite::pe::imports::Import::ByName { hint, name } => {
                            println!("  {} @ RVA 0x{:08X} VA 0x{:08X} (hint {})", 
                                     name.to_str()?, rva, va, hint)
                        }
                        pelite::pe::imports::Import::ByOrdinal { ord } => {
                            println!("  Ordinal {} @ RVA 0x{:08X} VA 0x{:08X}", ord, rva, va)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_relative_rip_target(cs: &Capstone, insn: &Insn) -> Option<u64> {
    let detail = cs.insn_detail(insn).unwrap();
    let arch_detail = detail.arch_detail();
    let x86_detail = arch_detail.x86().unwrap();

    for operand in x86_detail.operands() {
        match operand.op_type {
            capstone::arch::x86::X86OperandType::Mem(mem) => {
                if mem.base().0 == X86Reg::X86_REG_RIP as u16 {
                    let next_rip = insn.address() + insn.bytes().len() as u64;
                    return Some((next_rip as i64 + mem.disp()) as u64);
                }

                return None
            },
            _ => continue,
        }
    }

    None
}