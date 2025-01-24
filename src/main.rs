use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::Verbosity;

use anyhow::Result;
use tracing_log::AsTrace;

/* -------------------------------------------- CLI -------------------------------------------- */

/// The pixi-diff CLI.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// First lockfile to be compared.
    minus_file: PathBuf,

    /// Second lockfile to be compared.
    plus_file: PathBuf,

    #[command(flatten)]
    verbose: Verbosity,
}

/* -------------------------------------------- MAIN ------------------------------------------- */

/// The main entrypoint for the pixi-diff CLI.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(cli.verbose.log_level_filter().as_trace())
        .init();

    tracing::debug!("Starting pixi-diff CLI");
    tracing::debug!("Parsed CLI options: {:?}", cli);

    

    Ok(())
}
