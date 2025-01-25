use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::Verbosity;

use miette::IntoDiagnostic;
use tracing_log::AsTrace;

use rattler_lock::LockFile;

use pixi::{
    diff::{LockFileDiff, LockFileJsonDiff},
    Project,
};

/* -------------------------------------------- CLI -------------------------------------------- */

/// The pixi-diff CLI.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// First lockfile to be compared.
    minus_file: PathBuf,

    /// Second lockfile to be compared.
    plus_file: PathBuf,

    /// Pixi manifest file
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
        .with_max_level(cli.verbose.log_level_filter().as_trace())
        .init();

    tracing::debug!("Starting pixi-diff CLI");
    tracing::debug!("Parsed CLI options: {:?}", cli);

    let minus_lockfile = LockFile::from_path(&cli.minus_file).into_diagnostic()?;
    let plus_lockfile = LockFile::from_path(&cli.plus_file).into_diagnostic()?;

    let project = Project::load_or_else_discover(cli.manifest_path.as_deref())?;

    let diff = LockFileDiff::from_lock_files(&minus_lockfile, &plus_lockfile);
    let json_diff = LockFileJsonDiff::new(&project, diff);
    let json = serde_json::to_string_pretty(&json_diff).expect("failed to convert to json");

    println!("{}", json);

    Ok(())
}
