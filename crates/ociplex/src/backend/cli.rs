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
        // See https://github.com/opencontainers/runc/blob/main/man/runc-create.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("create".into());

        if args.bundle.as_os_str() != "." {
            backargs.push("--bundle".into());
            backargs.push(args.bundle.into_os_string());
        }

        if let Some(consock) = args.console_socket {
            backargs.push("--console-socket".into());
            backargs.push(consock.into_os_string());
        }

        if let Some(pidfile) = args.pid_file {
            backargs.push("--pid-file".into());
            backargs.push(pidfile.into_os_string());
        }

        if args.no_pivot {
            backargs.push("--no-pivot".into())
        }

        if args.no_new_keyring {
            backargs.push("--no-new-keyring".into());
        }

        if args.preserve_fds > 0 {
            backargs.push("--preserve-fds".into());
            backargs.push(args.preserve_fds.to_string().into())
        }

        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn start(&self, args: liboci_cli::Start) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-start.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("start".into());
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn kill(&self, args: liboci_cli::Kill) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-kill.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("kill".into());
        if args.all {
            backargs.push("--all".into())
        }
        backargs.push(args.container_id.into());
        backargs.push(args.signal.into());

        self.invoke(backargs)
    }

    fn delete(&self, args: liboci_cli::Delete) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-delete.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("delete".into());
        if args.force {
            backargs.push("--force".into())
        }
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn state(&self, args: liboci_cli::State) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-state.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("state".into());
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    // Common non-standard commands (from liboci_cli::CommonCmd)
    fn checkpoint(&self, args: liboci_cli::Checkpoint) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-checkpoint.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("checkpoint".into());

        if args.image_path.as_os_str() != "checkpoint" {
            backargs.push("--image-path".into());
            backargs.push(args.image_path.into_os_string())
        }
        if let Some(work_path) = args.work_path {
            backargs.push("--work-path".into());
            backargs.push(work_path.into_os_string())
        }
        if let Some(parent_path) = args.parent_path {
            backargs.push("--parent-path".into());
            backargs.push(parent_path.into_os_string())
        }
        if args.leave_running {
            backargs.push("--leave-running".into())
        }
        if args.tcp_established {
            backargs.push("--tcp-established".into())
        }
        if args.ext_unix_sk {
            backargs.push("--ext-unix-sk".into())
        }
        if args.shell_job {
            backargs.push("--shell-job".into())
        }
        if args.lazy_pages {
            backargs.push("--lazy-pages".into())
        }
        if let Some(status_fd) = args.status_fd {
            backargs.push("--status-fd".into());
            backargs.push(status_fd.to_string().into())
        }
        if let Some(page_server) = args.page_server {
            backargs.push("--page-server".into());
            backargs.push(page_server.into())
        }
        if args.file_locks {
            backargs.push("--file-locks".into())
        }
        if args.pre_dump {
            backargs.push("--pre-dump".into())
        }
        if let Some(cgroups_mode) = args.manage_cgroups_mode {
            backargs.push("--manage-cgroups-mode".into());
            backargs.push(cgroups_mode.into())
        }
        if args.empty_ns {
            backargs.push("--empty-ns".into())
        }
        if args.auto_dedup {
            backargs.push("--auto-dedup".into())
        }

        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn events(&self, args: liboci_cli::Events) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-events.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("events".into());

        if args.interval != 5 {
            backargs.push("--interval".into());
            backargs.push(args.interval.to_string().into());
        }
        if args.stats {
            backargs.push("--stats".into())
        }
        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }

    fn exec(&self, args: liboci_cli::Exec) -> Result<()> {
        // See https://github.com/opencontainers/runc/blob/main/man/runc-exec.8.md
        let mut backargs = Vec::<OsString>::new();

        backargs.push("exec".into());

        if let Some(consock) = args.console_socket {
            backargs.push("--console-socket".into());
            backargs.push(consock.into());
        }

        if let Some(cwd) = args.cwd {
            backargs.push("--cwd".into());
            backargs.push(cwd.into());
        }

        for (key, val) in args.env {
            backargs.push("--env".into());
            backargs.push(format!("{}={}", key, val).into());
        }

        if args.tty {
            backargs.push("--tty".into());
        }

        if let Some((uid, ogid)) = args.user {
            backargs.push("--user".into());
            if let Some(gid) = ogid {
                backargs.push(format!("{}:{}", uid, gid).into())
            } else {
                backargs.push(uid.to_string().into())
            }
        }

        for gid in args.additional_gids {
            backargs.push("--additional-gids".into());
            backargs.push(gid.to_string().into())
        }

        if let Some(process) = args.process {
            backargs.push("--process".into());
            backargs.push(process.into());
        }

        if args.detach {
            backargs.push("--detach".into());
        }

        if let Some(pidfile) = args.pid_file {
            backargs.push("--pid-file".into());
            backargs.push(pidfile.into_os_string());
        }

        if let Some(label) = args.process_label {
            backargs.push("--process-label".into());
            backargs.push(label.into())
        }

        if let Some(profile) = args.apparmor {
            backargs.push("--apparmor".into());
            backargs.push(profile.into())
        }

        if args.no_new_privs {
            backargs.push("--no-new-privs".into());
        }

        for cap in args.cap {
            backargs.push("--cap".into());
            backargs.push(cap.into())
        }

        if args.preserve_fds > 0 {
            backargs.push("--preserve-fds".into());
            backargs.push(args.preserve_fds.to_string().into())
        }

        if args.ignore_paused {
            backargs.push("--ignore-paused".into())
        }

        if let Some(cgroup) = args.cgroup {
            backargs.push("--cgroup".into());
            backargs.push(cgroup.into())
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

        if args.format != "table" {
            backargs.push("--format".into());
            backargs.push(args.format.into())
        }

        if args.quiet {
            backargs.push("-q".into())
        }

        self.invoke(backargs)
    }
    fn pause(&self, args: liboci_cli::Pause) -> Result<()> {
        let backargs: Vec<OsString> = vec!["pause".into(), args.container_id.into()];

        self.invoke(backargs)
    }
    fn ps(&self, args: liboci_cli::Ps) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("ps".into());

        if args.format != "table" {
            backargs.push("--format".into());
            backargs.push(args.format.into())
        }

        backargs.push(args.container_id.into());

        for opt in args.ps_options {
            backargs.push(opt.into())
        }

        self.invoke(backargs)
    }
    fn resume(&self, args: liboci_cli::Resume) -> Result<()> {
        let backargs: Vec<OsString> = vec!["resume".into(), args.container_id.into()];
        self.invoke(backargs)
    }
    fn run(&self, args: liboci_cli::Run) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("run".into());

        if args.bundle.as_os_str() != "." {
            backargs.push("--bundle".into());
            backargs.push(args.bundle.into_os_string())
        }

        if let Some(path) = args.console_socket {
            backargs.push("--console-socket".into());
            backargs.push(path.into_os_string())
        }

        if args.detach {
            backargs.push("--detach".into())
        }

        if let Some(pidfile) = args.pid_file {
            backargs.push("--pid-file".into());
            backargs.push(pidfile.into_os_string())
        }

        if args.no_subreaper {
            backargs.push("--no-subreaper".into())
        }

        if args.no_pivot {
            backargs.push("--no-pivot".into())
        }

        if args.no_new_keyring {
            backargs.push("--no-new-keyring".into())
        }

        if args.preserve_fds > 0 {
            backargs.push("--preserve-fds".into());
            backargs.push(args.preserve_fds.to_string().into())
        }

        if args.keep {
            backargs.push("--keep".into())
        }

        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }
    fn update(&self, args: liboci_cli::Update) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("update".into());

        if let Some(resources) = args.resources {
            backargs.push("--resources".into());
            backargs.push(resources.into_os_string())
        }

        if let Some(weight) = args.blkio_weight {
            backargs.push("--blkio-weight".into());
            backargs.push(weight.to_string().into())
        }

        if let Some(period) = args.cpu_period {
            backargs.push("--cpu-period".into());
            backargs.push(period.to_string().into())
        }

        if let Some(quota) = args.cpu_quota {
            backargs.push("--cpu-quota".into());
            backargs.push(quota.to_string().into())
        }

        if let Some(period) = args.cpu_rt_period {
            backargs.push("--cpu-rt-period".into());
            backargs.push(period.to_string().into())
        }

        if let Some(runtime) = args.cpu_rt_runtime {
            backargs.push("--cpu-rt-runtime".into());
            backargs.push(runtime.to_string().into())
        }

        if let Some(share) = args.cpu_share {
            backargs.push("--cpu-share".into());
            backargs.push(share.to_string().into())
        }

        if let Some(cpus) = args.cpuset_cpus {
            backargs.push("--cpuset-cpus".into());
            backargs.push(cpus.into())
        }

        if let Some(mems) = args.cpuset_mems {
            backargs.push("--cpuset-mems".into());
            backargs.push(mems.into())
        }

        if let Some(mem) = args.memory {
            backargs.push("--memory".into());
            backargs.push(mem.to_string().into())
        }

        if let Some(mem) = args.memory_reservation {
            backargs.push("--memory-reservation".into());
            backargs.push(mem.to_string().into())
        }

        if let Some(mem) = args.memory_swap {
            backargs.push("--memory-swap".into());
            backargs.push(mem.to_string().into())
        }

        if let Some(limit) = args.pids_limit {
            backargs.push("--pids-limit".into());
            backargs.push(limit.to_string().into())
        }

        if let Some(schema) = args.l3_cache_schema {
            backargs.push("--l3-cache-schema".into());
            backargs.push(schema.into())
        }

        if let Some(schema) = args.mem_bw_schema {
            backargs.push("--mem-bw-schema".into());
            backargs.push(schema.into())
        }

        backargs.push(args.container_id.into());

        self.invoke(backargs)
    }
    fn spec(&self, args: liboci_cli::Spec) -> Result<()> {
        let mut backargs = Vec::<OsString>::new();

        backargs.push("spec".into());

        self.invoke(backargs)
    }
}
