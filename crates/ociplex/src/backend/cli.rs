use std::ffi::OsString;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};

use liboci_cli::GlobalOpts;

use super::Backend;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn instantiate(self, global: GlobalOpts) -> Box<dyn Backend> {
        Box::new(CliBackend::new(self.path, global))
    }
}

#[derive(Debug)]
struct CliBackend {
    path: PathBuf,
    global_opts: Vec<OsString>,
}

impl CliBackend {
    fn new(path: PathBuf, global: GlobalOpts) -> Self {
        let mut opts = Vec::<OsString>::new();

        if global.debug {
            opts.push("--debug".into());
        }

        if let Some(logfile) = global.log {
            opts.push("--log".into());
            opts.push(logfile.into_os_string());
        }

        if let Some(format) = global.log_format {
            opts.push("--log-format".into());
            opts.push(format.into());
        }

        if let Some(root) = global.root {
            opts.push("--root".into());
            opts.push(root.into_os_string());
        }

        if global.systemd_cgroup {
            opts.push("--systemd-cgroup".into());
        }

        CliBackend {
            path,
            global_opts: opts,
        }
    }

    fn invoke(&self, args: impl IntoIterator<Item = OsString>) -> Result<()> {
        let status = Command::new(&self.path)
            .args(&self.global_opts)
            .args(args)
            .status()?;

        if status.success() {
            return Ok(());
        }

        Err(if let Some(sig) = status.signal() {
            anyhow!("Backend CLI terminated with signal {:?}", sig)
        } else if let Some(code) = status.code() {
            anyhow!("Backend CLI failed with status code {}", code)
        } else {
            anyhow!("Unidentified failure in backend CLI")
        })
    }
}

impl Backend for CliBackend {
    // Standard commands (from liboci_cli::StandardCmd)
    fn create(&self, args: liboci_cli::Create) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("create".into());

        if let Some(pidfile) = args.pid_file {
            backargs.push("--pid-file".into());
            backargs.push(pidfile.into_os_string());
        }

        backargs.push("--bundle".into());
        backargs.push(args.bundle.into_os_string());

        if let Some(consock) = args.console_socket {
            backargs.push("--console-socket".into());
            backargs.push(consock.into_os_string());
        }

        backargs.push("--preserve-fds".into());
        backargs.push(format!("{}", args.preserve_fds).into());

        if args.no_new_keyring {
            backargs.push("--no-new-keyring".into());
        }

        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn start(&self, args: liboci_cli::Start) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("start".into());
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn kill(&self, args: liboci_cli::Kill) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("kill".into());
        backargs.push(args.container_id.into());
        backargs.push(args.signal.into());

        self.invoke(backargs)
    }

    fn delete(&self, args: liboci_cli::Delete) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("delete".into());
        if args.force {
            backargs.push("--force".into())
        }
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn state(&self, args: liboci_cli::State) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("state".into());
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    // Common non-standard commands (from liboci_cli::CommonCmd)
    fn checkpoint(&self, args: liboci_cli::Checkpoint) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("checkpoint".into());

        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }
    fn events(&self, args: liboci_cli::Events) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("events".into());

        self.invoke(backargs)
    }

    fn exec(&self, args: liboci_cli::Exec) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("exec".into());
        if let Some(consock) = args.console_socket {
            backargs.push("--console-socket".into());
            backargs.push(consock.into());
        }

        if args.tty {
            backargs.push("--tty".into());
        }

        if let Some(cwd) = args.cwd {
            backargs.push("--cwd".into());
            backargs.push(cwd.into());
        }

        if let Some(pidfile) = args.pid_file {
            backargs.push("--pid-file".into());
            backargs.push(pidfile.into());
        }

        for (key, val) in args.env {
            backargs.push("--env".into());
            backargs.push(format!("{}={}", key, val).into());
        }

        if args.no_new_privs {
            backargs.push("--no-new-privs".into());
        }

        if let Some(process) = args.process {
            backargs.push("--process".into());
            backargs.push(process.into());
        }

        if args.detach {
            backargs.push("--detach".into());
        }

        backargs.push(args.container_id.into());
        for a in args.command {
            backargs.push(a.into());
        }

        self.invoke(backargs)
    }

    fn list(&self, args: liboci_cli::List) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("list".into());

        self.invoke(backargs)
    }
    fn pause(&self, args: liboci_cli::Pause) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("pause".into());

        self.invoke(backargs)
    }
    fn ps(&self, args: liboci_cli::Ps) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("ps".into());

        self.invoke(backargs)
    }
    fn resume(&self, args: liboci_cli::Resume) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("resume".into());

        self.invoke(backargs)
    }
    fn run(&self, args: liboci_cli::Run) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("run".into());

        self.invoke(backargs)
    }
    fn update(&self, args: liboci_cli::Update) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("update".into());

        self.invoke(backargs)
    }
    fn spec(&self, args: liboci_cli::Spec) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("spec".into());

        self.invoke(backargs)
    }
}
