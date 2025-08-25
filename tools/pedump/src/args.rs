use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    #[arg(long, short, required = true)]
    pub input: String,
    
}