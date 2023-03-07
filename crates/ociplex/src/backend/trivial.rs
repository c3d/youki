use anyhow::{anyhow, Result};

use liboci_cli::GlobalOpts;

use super::Backend;

#[derive(Debug, serde::Deserialize)]
pub struct Config {}

impl Config {
    pub fn instantiate(self, global: GlobalOpts) -> Box<dyn Backend> {
        dbg!(global);
        Box::new(TrivialBackend {})
    }
}

#[derive(Debug)]
struct TrivialBackend {}

impl Backend for TrivialBackend {
    // All standard commands (liboci_cli::StandardCmd)
    fn create(&self, args: liboci_cli::Create) -> Result<()> {
        Err(anyhow!("trivial {:?}", args))
    }

    fn start(&self, args: liboci_cli::Start) -> Result<()> {
        Err(anyhow!("trivial {:?}", args))
    }

    fn kill(&self, args: liboci_cli::Kill) -> Result<()> {
        Err(anyhow!("trivial {:?}", args))
    }

    fn delete(&self, args: liboci_cli::Delete) -> Result<()> {
        Err(anyhow!("trivial {:?}", args))
    }

    fn state(&self, args: liboci_cli::State) -> Result<()> {
        Err(anyhow!("trivial {:?}", args))
    }

    // All common but non-standard commands (liboci_cli::CommonCmd)
    fn checkpoint(&self, args: liboci_cli::Checkpoint) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn events(&self, args: liboci_cli::Events) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn exec(&self, args: liboci_cli::Exec) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn features(&self, args: liboci_cli::Features) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn list(&self, args: liboci_cli::List) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn pause(&self, args: liboci_cli::Pause) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn ps(&self, args: liboci_cli::Ps) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn resume(&self, args: liboci_cli::Resume) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn run(&self, args: liboci_cli::Run) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn update(&self, args: liboci_cli::Update) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
    fn spec(&self, args: liboci_cli::Spec) -> Result<()> {
        Err(anyhow!("trivial: {:?}", args))
    }
}
