use std::fmt::Debug;

use anyhow::{anyhow, Result};

use liboci_cli::{CommonCmd, GlobalOpts, StandardCmd};

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
    fn standard_command(&self, cmd: liboci_cli::StandardCmd) -> Result<()> {
        match cmd {
            StandardCmd::Create(args) => self.create(args),
            StandardCmd::Start(args) => self.start(args),
            StandardCmd::Kill(args) => self.kill(args),
            StandardCmd::Delete(args) => self.delete(args),
            StandardCmd::State(args) => self.state(args),
        }
    }

    fn common_command(&self, cmd: liboci_cli::CommonCmd) -> Result<()> {
        match cmd {
            CommonCmd::Checkpointt(args) => self.checkpoint(args),
            CommonCmd::Events(args) => self.events(args),
            CommonCmd::Exec(args) => self.exec(args),
            CommonCmd::List(args) => self.list(args),
            CommonCmd::Pause(args) => self.pause(args),
            CommonCmd::Ps(args) => self.ps(args),
            CommonCmd::Resume(args) => self.resume(args),
            CommonCmd::Run(args) => self.run(args),
            CommonCmd::Update(args) => self.update(args),
            CommonCmd::Spec(args) => self.spec(args),
        }
    }

    // StandardCmd in liboci-cli
    fn create(&self, args: liboci_cli::Create) -> Result<()>;
    fn start(&self, args: liboci_cli::Start) -> Result<()>;
    fn kill(&self, args: liboci_cli::Kill) -> Result<()>;
    fn delete(&self, args: liboci_cli::Delete) -> Result<()>;
    fn state(&self, args: liboci_cli::State) -> Result<()>;

    // CommonCmd in liboci-cli are not implemented by default
    fn checkpoint(&self, args: liboci_cli::Checkpoint) -> Result<()> {
        Err(anyhow!("checkpoint subcommand unimplemented: {:?}", args))
    }
    fn events(&self, args: liboci_cli::Events) -> Result<()> {
        Err(anyhow!("events subcommand unimplemented: {:?}", args))
    }
    fn exec(&self, args: liboci_cli::Exec) -> Result<()> {
        Err(anyhow!("exec subcommand unimplemented: {:?}", args))
    }
    fn list(&self, args: liboci_cli::List) -> Result<()> {
        Err(anyhow!("list subcommand unimplemented: {:?}", args))
    }
    fn pause(&self, args: liboci_cli::Pause) -> Result<()> {
        Err(anyhow!("pause subcommand unimplemented: {:?}", args))
    }
    fn ps(&self, args: liboci_cli::Ps) -> Result<()> {
        Err(anyhow!("ps subcommand unimplemented: {:?}", args))
    }
    fn resume(&self, args: liboci_cli::Resume) -> Result<()> {
        Err(anyhow!("resume subcommand unimplemented: {:?}", args))
    }
    fn run(&self, args: liboci_cli::Run) -> Result<()> {
        Err(anyhow!("run subcommand unimplemented: {:?}", args))
    }
    fn update(&self, args: liboci_cli::Update) -> Result<()> {
        Err(anyhow!("update subcommand unimplemented: {:?}", args))
    }
    fn spec(&self, args: liboci_cli::Spec) -> Result<()> {
        Err(anyhow!("spec subcommand unimplemented: {:?}", args))
    }
}
