use std::collections::HashMap;

use clap::crate_version;
use std::path::Path;
use std::sync::Arc;
use tokio::task::JoinHandle;

use colored::Colorize;

use tracing::trace;
use TheMill_internal::config::Config;
use TheMill_internal::TheMill_println;
use watchexec_supervisor::command::{Command, Program};
use watchexec_supervisor::job::{Job, start_job};

#[cfg(target_os = "windows")]
const DEV_WATCH_BIN_SRC: &str = "node_modules\\.bin\\TheMill-dev-watch.cmd";
#[cfg(target_os = "windows")]
const DEV_SSR_BIN_SRC: &str = "node_modules\\.bin\\TheMill-dev-ssr.cmd";

#[cfg(not(target_os = "windows"))]
const DEV_WATCH_BIN_SRC: &str = "node_modules/.bin/TheMill-dev-watch";
#[cfg(not(target_os = "windows"))]
const DEV_SSR_BIN_SRC: &str = "node_modules/.bin/TheMill-dev-ssr";

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ProcessId {
    WatchReactSrc,
    RunRustDevServer,
    BuildRustSrc,
    BuildReactSSRSrc,
}

// This struct manages all the processes spawned by the dev server
// That are not the dev server itself
#[derive(Debug)]
pub struct ProcessManager {
    processes: HashMap<ProcessId, (Job, JoinHandle<()>)>,
}

impl ProcessManager {
    pub fn new() -> Self {
        trace!("Creating process manager");
        if !Path::new(DEV_SSR_BIN_SRC).exists() {
            eprintln!("Failed to find script to run dev watch. Please run `npm install`");
            std::process::exit(1);
        }
        let mut processes = HashMap::new();
        processes.insert(
            ProcessId::WatchReactSrc,
            start_supervisor_job(DEV_WATCH_BIN_SRC, Vec::new()),
        );

        processes.insert(
            ProcessId::RunRustDevServer,
            start_supervisor_job("cargo", vec!["run", "-q"]),
        );

        processes.insert(
            ProcessId::BuildRustSrc,
            start_supervisor_job("cargo", vec!["build", "-q"]),
        );

        processes.insert(
            ProcessId::BuildReactSSRSrc,
            start_supervisor_job(DEV_SSR_BIN_SRC, Vec::new()),
        );

        Self { processes }
    }

    pub fn start_process(&mut self, id: ProcessId) {
        trace!("start process {:?}", id);
        if let Some((job, _)) = self.processes.get(&id) {
            job.start();
        }
    }

    // Start all the processes needed for the dev server
    pub async fn start_dev_processes(&mut self) {
        trace!("Starting dev processes");
        self.start_process(ProcessId::WatchReactSrc);
        self.start_process(ProcessId::BuildRustSrc);
        self.start_process(ProcessId::BuildReactSSRSrc);

        self.wait_for_process(ProcessId::BuildRustSrc).await;
        self.wait_for_process(ProcessId::BuildReactSSRSrc).await;

        // This needs to start after having built the rust and react sources
        self.start_process(ProcessId::RunRustDevServer);
    }

    pub fn log_server_address(&self, config: Config) {
        let server_address = format!("{}:{}", config.server.host, config.server.port);
        // Format the server address as a valid URL so that it becomes clickable in the CLI
        // @see https://github.com/TheJackMan33/TheMill/issues/460
        let server_base_url = format!("http://{}", server_address);

        println!();
        TheMill_println!("⚡ TheMill v{}", crate_version!());

        TheMill_println!("Development server at: {}\n", server_base_url.blue().bold());
        if let Some(origin) = config.server.origin {
            TheMill_println!("Origin: {}\n", origin.blue().bold());
        }
    }

    pub fn restart_process(&mut self, id: ProcessId) {
        trace!("Restarting process {:?}", id);
        if let Some((job, _)) = self.processes.get(&id) {
            job.stop();
            job.start();
        }
    }

    pub async fn wait_for_process(&mut self, id: ProcessId) {
        trace!("Waiting for process {:?}", id);
        if let Some((job, _)) = self.processes.get(&id) {
            job.to_wait().await;
        }
    }

    pub fn abort_all(&mut self) {
        trace!("Aborting all processes");
        for (_, (job, handle)) in self.processes.iter() {
            job.stop();
            handle.abort();
        }
    }
}

fn start_supervisor_job(prog: &str, args: Vec<&str>) -> (Job, JoinHandle<()>) {
    start_job(Arc::new(Command {
        program: Program::Exec {
            prog: prog.into(),
            args: args.into_iter().map(|arg| arg.to_string()).collect(),
        },
        options: Default::default(),
    }))
}
