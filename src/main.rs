use clap::Parser;
use clap_verbosity_flag::Verbosity;

use std::path::PathBuf;

use pixi_diff_cli::{Input, diff};

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
