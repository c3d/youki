use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{crate_version, Parser};

use liboci_cli::{GlobalOpts, StandardCmd};

mod backend;

#[derive(Parser, Debug)]
#[clap(version = crate_version!())]
struct Opts {
    #[clap(long)]
    backend: PathBuf,

    #[clap(flatten)]
    global: GlobalOpts,

    #[clap(subcommand)]
    subcmd: StandardCmd,
}

fn main() -> Result<()> {
    let opts = Opts::try_parse()?;

    let config = fs::read_to_string(&opts.backend).context("Reading backend config")?;
    let config: backend::Config = toml::from_str(&config).context("Parsing backend config")?;

    let backend = config.instantiate(opts.global);
    backend.standard_command(opts.subcmd)?;

    Ok(())
}
