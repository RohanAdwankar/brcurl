use clap::Parser;

fn main() -> anyhow::Result<()> {
    brcurl::run(brcurl::Cli::parse())
}
