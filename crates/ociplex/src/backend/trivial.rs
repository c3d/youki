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
}
