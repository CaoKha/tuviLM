//! Dev commands for the TuviLM workspace.
//!
//! - `cargo up`: start Postgres, apply migrations, then run `topcoat dev`.
//! - `cargo down`: stop Postgres. The data stays in the Docker volume.
//! - `cargo tuvi ask "..."`: ask the LLM one question, to check the key.

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use clap::{Parser, Subcommand};

/// Dev commands for the TuviLM workspace.
#[derive(Parser)]
#[command(
    bin_name = "cargo tuvi",
    after_help = "Shortcuts: `cargo up`, `cargo down`."
)]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Subcommand)]
enum Task {
    /// Start Postgres, apply migrations, then run `topcoat dev`.
    Up,
    /// Stop Postgres. The data stays in the Docker volume.
    Down,
    /// Ask the LLM one question and print the answer as it streams.
    ///
    /// Use it to check your key. The provider comes from OPENCODE_API_KEY or
    /// OPENROUTER_API_KEY (see `.env.example`).
    Ask {
        /// The question.
        question: String,
    },
}

fn main() -> ExitCode {
    // A missing `.env` is fine: the variables can come from the shell.
    let _ = dotenvy::from_path(workspace_root().join(".env"));

    let result = match Cli::parse().command {
        Task::Up => up(),
        Task::Down => down(),
        Task::Ask { question } => ask(&question),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn up() -> Result<(), String> {
    let root = workspace_root();
    let web = root.join("crates/tuvi-web");

    if std::env::var_os("DATABASE_URL").is_some() {
        step("Using DATABASE_URL: not starting Postgres");
    } else if postgres_is_up() {
        step("Postgres is already running");
    } else {
        step("Starting Postgres");
        // `--wait` returns when the health check in docker-compose.yaml passes.
        run(docker()?
            .args(["compose", "up", "-d", "--wait", "postgres"])
            .current_dir(&root))?;
    }

    step("Applying migrations");
    // Toasty reads `toasty/` from the current folder, so run it from the web crate.
    run(Command::new(cargo())
        .args([
            "run",
            "-q",
            "-p",
            "tuvi-web",
            "--",
            "toasty",
            "migration",
            "apply",
        ])
        .current_dir(&web))?;

    step("Starting topcoat dev (Ctrl-C to stop)");
    let mut dev = Command::new("topcoat");
    dev.arg("dev").current_dir(&web);
    exec(dev)
        .map_err(|error| format!("{error}\nInstall the Topcoat CLI: cargo install topcoat-cli"))
}

fn ask(question: &str) -> Result<(), String> {
    use futures_util::StreamExt;
    use std::io::Write;
    use tuvi_llm::{LlmClient, LlmConfig, Message, Session};

    let config = LlmConfig::from_env().map_err(|error| error.to_string())?;
    step(&format!("{} · {}", config.provider, config.model));
    let client = LlmClient::new(config).map_err(|error| error.to_string())?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())?;
    runtime.block_on(async {
        let session = Session::new();
        let mut answer = client
            .stream(&session, &[Message::user(question)])
            .await
            .map_err(|error| error.to_string())?;
        let mut stdout = std::io::stdout().lock();
        while let Some(piece) = answer.next().await {
            let piece = piece.map_err(|error| error.to_string())?;
            let _ = stdout.write_all(piece.as_bytes());
            let _ = stdout.flush();
        }
        let _ = writeln!(stdout);
        Ok(())
    })
}

fn down() -> Result<(), String> {
    step("Stopping Postgres");
    run(docker()?
        .args(["compose", "stop", "postgres"])
        .current_dir(workspace_root()))
}

/// A `docker` command. When the Docker socket needs root (the user is not in
/// the `docker` group), the command runs through `sudo`, which asks for the
/// password in the terminal.
fn docker() -> Result<Command, String> {
    let probe = Command::new("docker")
        .args(["version", "--format", "{{.Server.Version}}"])
        .output()
        .map_err(|error| format!("cannot run `docker`: {error}. Is Docker installed?"))?;
    if probe.status.success() {
        return Ok(Command::new("docker"));
    }

    let stderr = String::from_utf8_lossy(&probe.stderr);
    if !stderr.contains("permission denied") {
        return Err(format!(
            "Docker is not ready. Is the Docker daemon running?\n{}",
            stderr.trim()
        ));
    }
    step("Docker needs root: running it with sudo (enter your password if asked)");
    let mut command = Command::new("sudo");
    command.arg("docker");
    Ok(command)
}

/// Whether something accepts connections on the default Postgres port, the one
/// in docker-compose.yaml and in `crates/tuvi-web/src/db.rs`.
fn postgres_is_up() -> bool {
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 5432));
    std::net::TcpStream::connect_timeout(&address, std::time::Duration::from_millis(300)).is_ok()
}

/// The workspace root. This crate is at `<root>/crates/tuvi-cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The `cargo` that runs this command, so the same toolchain builds `web`.
fn cargo() -> std::ffi::OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}

fn step(message: &str) {
    eprintln!("\x1b[1;32m==>\x1b[0m {message}");
}

/// Runs a command to the end. Its output goes to the terminal.
fn run(command: &mut Command) -> Result<(), String> {
    let name = display(command);
    let status = command
        .status()
        .map_err(|error| format!("cannot run `{name}`: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`{name}` failed ({status})"))
    }
}

/// Replaces this process with `command`, so Ctrl-C goes straight to it.
#[cfg(unix)]
fn exec(mut command: Command) -> Result<(), String> {
    use std::os::unix::process::CommandExt;
    let name = display(&command);
    // `exec` returns only on failure.
    let error = command.exec();
    Err(format!("cannot run `{name}`: {error}"))
}

#[cfg(not(unix))]
fn exec(mut command: Command) -> Result<(), String> {
    run(&mut command)
}

fn display(command: &Command) -> String {
    std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(OsStr::to_string_lossy)
        .collect::<Vec<_>>()
        .join(" ")
}
