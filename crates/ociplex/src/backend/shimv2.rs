use containerd_shim_protos as shim;

use containerd_shim_protos as client;
use shim::ttrpc::context::Context;
use shim::{api, api::ConnectResponse, Client, TaskClient};

use std::ffi::OsString;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};

use liboci_cli::GlobalOpts;

use super::Backend;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    shim: PathBuf,
    socket: PathBuf,
    events: PathBuf,
}

impl Config {
    pub fn instantiate(self, opts: GlobalOpts) -> Box<dyn Backend> {
        Box::new(ShimV2Backend::new(
            self.shim,
            self.socket,
            self.events,
            opts,
        ))
    }
}

#[derive(Debug)]
struct ShimV2Backend {
    shim: PathBuf,
    socket: PathBuf,
    events: PathBuf,
    global_opts: GlobalOpts,
}

impl ShimV2Backend {
    fn new(shim: PathBuf, socket: PathBuf, events: PathBuf, global_opts: GlobalOpts) -> Self {
        ShimV2Backend {
            shim,
            socket,
            events,
            global_opts,
        }
    }

    fn launch(&self, socket_path: &str) -> Result<Client> {
        let mut cmdargs = Vec::<OsString>::new();

        cmdargs.push("start".into());
        cmdargs.push("-namespace".into());
        cmdargs.push("default".into());
        cmdargs.push("-address".into());
        cmdargs.push(self.socket.clone().into());
        cmdargs.push("-publish-binary".into());
        cmdargs.push(self.events.clone().into());

        let status = Command::new(&self.shim).args(cmdargs).status()?;

        if status.success() {
            return client::Client::connect(socket_path).map_err(anyhow::Error::from);
        }

        let path = &self.shim;
        Err(if let Some(sig) = status.signal() {
            anyhow!("ShimV2 backend {:?} terminated with signal {:?}", path, sig)
        } else if let Some(code) = status.code() {
            anyhow!("ShimV2 backend {:?} failed with status code {}", path, code)
        } else {
            anyhow!("Unidentified failure in ShimV2 backend")
        })
    }

    fn invoke(&self, pid: String) -> Result<(TaskClient, Context, ConnectResponse)> {
        let socket_path = self.socket.to_str().ok_or_else(|| {
            anyhow!(
                "ShimV2 socket path {} contains invalid characters",
                self.socket.display()
            )
        })?;

        let client = client::Client::connect(socket_path).or_else(|_| self.launch(socket_path))?;
        let task_client = client::TaskClient::new(client);
        let context = Context::default();
        let req = api::ConnectRequest {
            id: pid,
            ..Default::default()
        };
        let resp = task_client.connect(context.clone(), &req)?;
        Ok((task_client, context, resp))
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
