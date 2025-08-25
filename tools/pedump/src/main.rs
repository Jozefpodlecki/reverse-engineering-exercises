use anyhow::Result;
use clap::Parser;
use flexi_logger::Logger;

use crate::{args::CliArgs, dump::Dump};

mod args;
mod dump;
mod utils;

fn main() -> Result<()> {
    Logger::try_with_str("info")?.start()?;

    let args = CliArgs::parse();

    let dump = Dump::new(args.input.into());

    dump.run()?;

    Ok(())
}