use std::fmt::Debug;

use anyhow::Result;

use liboci_cli::{GlobalOpts, StandardCmd};

mod cli;
mod trivial;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "backend-type")]
pub enum Config {
    Trivial(trivial::Config),
    Cli(cli::Config),
}

impl Config {
    pub fn instantiate(self, global: GlobalOpts) -> Box<dyn Backend> {
        match self {
            Config::Trivial(c) => c.instantiate(global),
            Config::Cli(c) => c.instantiate(global),
        }
    }
}

pub trait Backend: Debug {
    fn create(&self, args: liboci_cli::Create) -> Result<()>;
    fn start(&self, args: liboci_cli::Start) -> Result<()>;
    fn kill(&self, args: liboci_cli::Kill) -> Result<()>;
    fn delete(&self, args: liboci_cli::Delete) -> Result<()>;
    fn state(&self, args: liboci_cli::State) -> Result<()>;

    fn standard_command(&self, cmd: liboci_cli::StandardCmd) -> Result<()> {
        match cmd {
            StandardCmd::Create(args) => self.create(args),
            StandardCmd::Start(args) => self.start(args),
            StandardCmd::Kill(args) => self.kill(args),
            StandardCmd::Delete(args) => self.delete(args),
            StandardCmd::State(args) => self.state(args),
        }
    }
}
