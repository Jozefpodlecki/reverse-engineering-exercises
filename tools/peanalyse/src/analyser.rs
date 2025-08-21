use std::{fs, path::PathBuf};
use anyhow::Result;
use capstone::arch::{x86::X86Insn, DetailsArchInsn};
use pelite::{pe::Pe, PeFile, Wrap};

use crate::{pe_info::{PEInfo, PEKind}, utils::{build_capstone, get_relative_rip_target, print_imports}};

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

    pub fn read_addr(&self, addr: u64, is_va: bool) -> Result<()> {
        let data = fs::read(&self.0)?;
        let file = PeFile::from_bytes(&data)?;

        match file {
            Wrap::T32(pe) => {
                let optional_header = pelite::pe32::Pe::optional_header(pe);
                let image_base = optional_header.ImageBase as u64;

                let rva = if is_va {
                    (addr - image_base) as u32
                } else {
                    addr as u32
                };

                let bytes = pelite::pe32::Pe::derva_slice(pe, rva, 4)?;
                let value = u32::from_le_bytes(bytes.try_into().unwrap()) as u64;
                println!("0x{:X} - 0x{:X}", addr, value);
            }
            Wrap::T64(pe) => {
                let optional_header = pe.optional_header();
                let image_base = optional_header.ImageBase;

                let rva = if is_va {
                    (addr - image_base) as u32
                } else {
                    addr as u32
                };

                let bytes = file.derva_slice(rva, 4)?;
                let value = u32::from_le_bytes(bytes.try_into().unwrap());
                println!("0x{:X} - 0x{:X}", addr, value);
            }
        };

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

            let bytes = file.derva_slice(rva, 15)?;
            let insns = cs.disasm_all(bytes, va)?;
            if insns.is_empty() {
                break;
            }

            let insn = &insns[0];
            let mnemonic = insn.mnemonic().unwrap_or("");
            let op_str = insn.op_str().unwrap_or("");

            match X86Insn::from(insn.id().0) {
                X86Insn::X86_INS_LEA | X86Insn::X86_INS_CALL => {
                    if let Some(target) = get_relative_rip_target(&cs, insn) {
                        println!(
                            "0x{:08X}: {:<10} {:<20} ; target = 0x{:X}",
                            insn.address(),
                            mnemonic,
                            op_str,
                            target
                        );
                    } else {
                        println!("0x{:08X}: {:<10} {}", insn.address(), mnemonic, op_str);
                    }
                },
                _ => {
                    println!("0x{:08X}: {:<10} {}", insn.address(), mnemonic, op_str);
                },
            }

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