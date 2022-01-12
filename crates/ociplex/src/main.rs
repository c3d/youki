use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{crate_version, Parser};

use liboci_cli::{GlobalOpts, StandardCmd};

mod backend;

#[derive(Parser, Debug)]
enum Subcommand {
    #[clap(flatten)]
    Standard(StandardCmd),
}

#[derive(Parser, Debug)]
#[clap(version = crate_version!())]
struct Opts {
    #[clap(long)]
    backend: PathBuf,

    #[clap(flatten)]
    global: GlobalOpts,

    #[clap(subcommand)]
    subcmd: Subcommand,
}

fn main() -> Result<()> {
    let opts = match Opts::try_parse() {
        Ok(opts) => opts,
        Err(e) => e.exit(),
    };

    let config = fs::read_to_string(&opts.backend).context("Reading backend config")?;
    let config: backend::Config = toml::from_str(&config).context("Parsing backend config")?;

    let backend = config.instantiate(opts.global);
    match opts.subcmd {
        Subcommand::Standard(std) => backend.standard_command(std)?,
    }

    Ok(())
}
