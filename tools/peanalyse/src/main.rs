use anyhow::Result;
use clap::Parser;

use crate::{analyser::Analyser, args::CliArgs};

mod pe_info;
mod args;
mod analyser;
mod utils;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let analyser = Analyser::new(args.input.into());

    if let Some(start_addr) = args.disassemble_from_addr {

        let mut stop_on_int3 = args.stop_on_int3;

        if args.disassemble_to_addr.is_none() {
            stop_on_int3 = true;
        }

        analyser.disassemble_range(
            start_addr,
            args.disassemble_to_addr,
            stop_on_int3)?;
        return Ok(());
    }

    if let Some(addr) = args.read_addr {
        analyser.read_addr(addr, true)?;
        return Ok(());
    }

    if args.iat_entries {
        analyser.iat_entries()?;
        return Ok(());
    }

    if args.summary {
        analyser.summary()?;
        return Ok(());
    }

    if args.entry {
        analyser.entry()?;
    }

    Ok(())
}
