use clap::Parser;
use clap_verbosity_flag::Verbosity;

use std::{
    env::current_dir,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use miette::IntoDiagnostic;
use pixi_core::Workspace;
use pixi_diff::{LockFileDiff, LockFileJsonDiff};
use pixi_manifest::{DiscoveryStart, WorkspaceDiscoverer};
use rattler_lock::LockFile;
use tracing::error;

/* -------------------------------------------- LIB -------------------------------------------- */

#[derive(Debug, Clone)]
pub enum Input {
    File(PathBuf),
    Stdin,
}

fn read_input(input: &Input) -> miette::Result<String> {
    match input {
        Input::File(path) => std::fs::read_to_string(path).into_diagnostic(),
        Input::Stdin => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .into_diagnostic()?;
            Ok(buffer)
        }
    }
}

pub fn diff(before: Input, after: Input, manifest_path: Option<&Path>) -> miette::Result<String> {
    let before_content = read_input(&before)?;
    let after_content = read_input(&after)?;

    let before_lockfile = LockFile::from_str(&before_content).into_diagnostic()?;
    let after_lockfile = LockFile::from_str(&after_content).into_diagnostic()?;

    let discover_start = match manifest_path {
        Some(path) if path.is_file() => DiscoveryStart::ExplicitManifest(path.to_path_buf()),
        Some(path) if path.is_dir() => DiscoveryStart::SearchRoot(path.to_path_buf()),
        _ => DiscoveryStart::SearchRoot(current_dir().into_diagnostic()?),
    };

    let workspace = match WorkspaceDiscoverer::new(discover_start).discover() {
        Ok(Some(manifests)) => {
            let manifest_path = manifests.value.workspace.provenance.path.clone();
            Some(Workspace::from_path(&manifest_path)?)
        }
        Ok(None) => None,
        Err(err) => {
            error!("Error discovering workspace: {err}");
            error!(
                "Skipping workspace discovery. This will result in explicit/implicit not being included in the diff."
            );
            None
        }
    };

    let diff = LockFileDiff::from_lock_files(&before_lockfile, &after_lockfile);
    let json_diff = LockFileJsonDiff::new(
        workspace.as_ref().map(|ws| ws.named_environments().clone()),
        diff,
    );
    Ok(serde_json::to_string_pretty(&json_diff).expect("failed to convert to json"))
}

/* -------------------------------------------- CLI -------------------------------------------- */

fn parse_input(s: &str) -> Result<Input, String> {
    if s == "-" {
        Ok(Input::Stdin)
    } else {
        Ok(Input::File(PathBuf::from(s)))
    }
}

/// The pixi-diff CLI.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// First lockfile to be compared.
    #[arg(long, short, value_parser = parse_input)]
    before: Option<Input>,
    /// Second lockfile to be compared.
    #[arg(long, short, value_parser = parse_input)]
    after: Option<Input>,

    // Positional args needed s.t. `pixi-diff old-file new-file` also works
    /// First lockfile to be compared.
    #[clap(name = "BEFORE", requires = "AFTER", conflicts_with_all = ["before", "after"])]
    before_positional: Option<PathBuf>,
    /// Second lockfile to be compared.
    #[clap(name = "AFTER", requires = "BEFORE")]
    after_positional: Option<PathBuf>,

    /// Pixi manifest file. Used to determine whether a dependency is explicit.
    #[arg(long)]
    manifest_path: Option<PathBuf>,

    #[command(flatten)]
    verbose: Verbosity,
}

/* -------------------------------------------- MAIN ------------------------------------------- */

/// The main entrypoint for the pixi-diff CLI.
fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(cli.verbose)
        .with_writer(std::io::stderr)
        .init();

    tracing::debug!("Starting pixi-diff CLI");
    tracing::debug!("Parsed CLI options: {:?}", cli);

    if cli.before.is_none()
        && cli.after.is_none()
        && cli.before_positional.is_none()
        && cli.after_positional.is_none()
    {
        miette::bail!("Either [BEFORE] or [AFTER] is required")
    }

    let (before, after) = if let (Some(before_positional), Some(after_positional)) =
        (cli.before_positional, cli.after_positional)
    {
        (
            Input::File(before_positional),
            Input::File(after_positional),
        )
    } else {
        (
            cli.before.unwrap_or(Input::Stdin),
            cli.after.unwrap_or(Input::Stdin),
        )
    };

    // ensure not both are stdin
    if matches!(before, Input::Stdin) && matches!(after, Input::Stdin) {
        miette::bail!("Cannot read both inputs from stdin");
    }
    tracing::debug!("Before: {:?}, After: {:?}", before, after);

    let json = diff(before, after, cli.manifest_path.as_deref())?;
    println!("{json}");

    Ok(())
}
