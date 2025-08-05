use std::{
    env::current_dir,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use miette::IntoDiagnostic;
use pixi::{
    diff::{LockFileDiff, LockFileJsonDiff},
    workspace::Workspace,
};
use pixi_manifest::{DiscoveryStart, WorkspaceDiscoverer};
use rattler_lock::LockFile;

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
            let manifest_path = &manifests.value.workspace.provenance.path;
            Some(Workspace::from_path(manifest_path)?)
        }
        _ => None,
    };

    let diff = LockFileDiff::from_lock_files(&before_lockfile, &after_lockfile);
    let json_diff = LockFileJsonDiff::new(workspace.as_ref(), diff);
    Ok(serde_json::to_string_pretty(&json_diff).expect("failed to convert to json"))
}
