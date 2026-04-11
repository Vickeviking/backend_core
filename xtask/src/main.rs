use anyhow::{Context, Result, bail};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };

    let workspace_root = workspace_root()?;

    match command.as_str() {
        "dev" => dev(&workspace_root),
        "ci" => ci(&workspace_root),
        "db-init" => db_init(&workspace_root),
        "redis-init" => redis_init(&workspace_root),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        other => bail!("Unknown xtask command: {other}"),
    }
}

fn workspace_root() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("Failed to determine the workspace root from xtask")
}

fn dev(workspace_root: &Path) -> Result<()> {
    ensure_web_dependencies(workspace_root)?;

    let script = r#"
set -euo pipefail
trap 'jobs -pr | xargs -r kill 2>/dev/null || true' INT TERM EXIT

./scripts/dev/dev.sh &
api_pid=$!

./scripts/dev/worker.sh &
worker_pid=$!

(cd apps/web && npm run dev) &
web_pid=$!

wait -n "$api_pid" "$worker_pid" "$web_pid"
status=$?

kill "$api_pid" "$worker_pid" "$web_pid" 2>/dev/null || true
wait "$api_pid" "$worker_pid" "$web_pid" 2>/dev/null || true

exit "$status"
"#;

    run_bash(workspace_root, script)
}

fn ci(workspace_root: &Path) -> Result<()> {
    run(
        workspace_root,
        "./scripts/ci-check.sh",
        std::iter::empty::<&str>(),
    )?;
    run(workspace_root.join("apps/web"), "npm", ["ci"])?;
    run(workspace_root.join("apps/web"), "npm", ["run", "build"])?;
    Ok(())
}

fn db_init(workspace_root: &Path) -> Result<()> {
    run(
        workspace_root,
        "./scripts/init_db.sh",
        std::iter::empty::<&str>(),
    )
}

fn redis_init(workspace_root: &Path) -> Result<()> {
    run(
        workspace_root,
        "./scripts/init_redis.sh",
        std::iter::empty::<&str>(),
    )
}

fn ensure_web_dependencies(workspace_root: &Path) -> Result<()> {
    let node_modules = workspace_root.join("apps/web/node_modules");
    if node_modules.exists() {
        return Ok(());
    }

    run(workspace_root.join("apps/web"), "npm", ["install"])
}

fn run_bash(workspace_root: &Path, script: &str) -> Result<()> {
    run(workspace_root, "bash", ["-lc", script])
}

fn run<I, S>(workdir: impl AsRef<Path>, program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(program)
        .args(args)
        .current_dir(workdir.as_ref())
        .status()
        .with_context(|| format!("Failed to start `{program}`"))?;

    ensure_success(program, status)
}

fn ensure_success(program: &str, status: ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        bail!("`{program}` exited with status {status}")
    }
}

fn print_help() {
    eprintln!(
        "\
Usage: cargo xtask <command>

Commands:
  dev         Start the local developer shell
  ci          Run backend checks and the web build
  db-init     Initialize Postgres for local development
  redis-init  Initialize Redis for local development
"
    );
}
