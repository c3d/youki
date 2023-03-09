use containerd_shim_protos as shim;

use shim::api;
use shim::ttrpc::context::Context;

use std::ffi::OsString;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};

use liboci_cli::GlobalOpts;

use super::Backend;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    binary_path: PathBuf,
    socket: PathBuf,
}

impl Config {
    pub fn instantiate(self, opts: GlobalOpts) -> Box<dyn Backend> {
        Box::new(ShimV2Backend::new(self.binary_path, self.socket, opts))
    }
}

#[derive(Debug)]
struct ShimV2Backend {
    path: PathBuf,
    socket: PathBuf,
    global_opts: GlobalOpts,
}

impl ShimV2Backend {
    fn new(path: PathBuf, socket: PathBuf, global_opts: GlobalOpts) -> Self {
        ShimV2Backend {
            path,
            socket,
            global_opts,
        }
    }

    fn launch(&self, args: impl IntoIterator<Item = OsString>) -> Result<()> {
        let status = Command::new(&self.path).args(args).status()?;

        if status.success() {
            return Ok(());
        }

        let path = &self.path;
        Err(if let Some(sig) = status.signal() {
            anyhow!("ShimV2 backend {:?} terminated with signal {:?}", path, sig)
        } else if let Some(code) = status.code() {
            anyhow!("ShimV2 backend {:?} failed with status code {}", path, code)
        } else {
            anyhow!("Unidentified failure in ShimV2 backend")
        })
    }
}

impl Backend for ShimV2Backend {
    // Standard commands (from liboci_cli::StandardCmd)
    fn create(&self, args: liboci_cli::Create) -> Result<()> {
        Ok(())
    }

    fn start(&self, args: liboci_cli::Start) -> Result<()> {
        Ok(())
    }

    fn kill(&self, args: liboci_cli::Kill) -> Result<()> {
        Ok(())
    }

    fn delete(&self, args: liboci_cli::Delete) -> Result<()> {
        Ok(())
    }

    fn state(&self, args: liboci_cli::State) -> Result<()> {
        Ok(())
    }

    // Common non-standard commands (from liboci_cli::CommonCmd)
    fn checkpoint(&self, args: liboci_cli::Checkpoint) -> Result<()> {
        Ok(())
    }

    fn events(&self, args: liboci_cli::Events) -> Result<()> {
        Ok(())
    }

    fn exec(&self, args: liboci_cli::Exec) -> Result<()> {
        Ok(())
    }

    fn features(&self, _args: liboci_cli::Features) -> Result<()> {
        Ok(())
    }

    fn list(&self, args: liboci_cli::List) -> Result<()> {
        Ok(())
    }

    fn pause(&self, args: liboci_cli::Pause) -> Result<()> {
        Ok(())
    }

    fn ps(&self, args: liboci_cli::Ps) -> Result<()> {
        Ok(())
    }

    fn resume(&self, args: liboci_cli::Resume) -> Result<()> {
        Ok(())
    }

    fn run(&self, args: liboci_cli::Run) -> Result<()> {
        Ok(())
    }

    fn update(&self, args: liboci_cli::Update) -> Result<()> {
        Ok(())
    }

    fn spec(&self, args: liboci_cli::Spec) -> Result<()> {
        Ok(())
    }
}
