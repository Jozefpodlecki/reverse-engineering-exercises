use std::{fs, path::PathBuf};
use anyhow::Result;
use capstone::{arch::{self, x86::X86Insn, BuildsCapstone, BuildsCapstoneSyntax}, Capstone};
use pelite::{pe::Pe, PeFile, Wrap};

use crate::{pe_info::{PEInfo, PEKind}, utils::{build_capstone, print_imports}};

pub struct Analyser(PathBuf);

impl Analyser {
    pub fn new(input: PathBuf) -> Self {
        Self(input)
    }

    pub fn iat_entries(&self) -> Result<()> {
        let data = fs::read(&self.0)?;
        let file = PeFile::from_bytes(&data)?;
        print_imports(file)?;

        Ok(())
    }

    pub fn disassemble_range(
        &self,
        start_va: u64,
        stop_va: Option<u64>,
        stop_on_int3: bool,
    ) -> Result<()> {
        let cs = build_capstone()?;
        let data = fs::read(&self.0)?;
        let file = PeFile::from_bytes(&data)?;

        let info = match file {
            Wrap::T32(pe) => PEInfo::from_pe32(pe)?,
            Wrap::T64(pe) => PEInfo::from_pe64(pe)?,
        };

        let mut va = start_va;
        loop {
            let rva = va.checked_sub(info.image_base)
                .ok_or_else(|| anyhow::anyhow!("Address below image base"))? as u32;

            // Get one instruction max 15 bytes (x86 instruction max length)
            let bytes = file.derva_slice(rva, 15)?;
            let insns = cs.disasm_all(bytes, va)?;
            if insns.is_empty() {
                break;
            }

            let insn = &insns[0];
            println!(
                "0x{:08X}: {:<10} {}",
                insn.address(),
                insn.mnemonic().unwrap_or(""),
                insn.op_str().unwrap_or("")
            );

            if stop_on_int3 && insn.bytes()[0] == 0xCC {
                break;
            }

            va = insn.address() + insn.bytes().len() as u64;
            if let Some(stop) = stop_va {
                if va >= stop {
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn entry(&self) -> Result<()> {
        let data = fs::read(&self.0)?;
        let file = PeFile::from_bytes(&data)?;

        let (kind, info) = match file {
            Wrap::T32(pe) => (PEKind::Pe32, PEInfo::from_pe32(pe)?),
            Wrap::T64(pe) => (PEKind::Pe64, PEInfo::from_pe64(pe)?),
        };

        println!("Architecture: {:?}", kind);
        println!("AddressOfEntryPoint (RVA): 0x{:08X}", info.address_of_entry_point);
        println!("ImageBase:                0x{:08X}", info.image_base);
        println!("EntryPoint (VA):          0x{:08X}", info.entry_va());

        Ok(())
    }

    pub fn summary(&self) -> Result<()> {
        let data = fs::read(&self.0)?;
        let file = PeFile::from_bytes(&data)?;

        let (kind, info) = match file {
            Wrap::T32(pe) => (PEKind::Pe32, PEInfo::from_pe32(pe)?),
            Wrap::T64(pe) => (PEKind::Pe64, PEInfo::from_pe64(pe)?),
        };

        println!("File: {}", self.0.file_name().unwrap().display());
        println!("Architecture: {:?}", kind);
        println!("== Optional Header Summary ==");
        println!("{info}");

        Ok(())
    }
}