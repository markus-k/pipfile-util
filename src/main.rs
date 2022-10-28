use clap::Parser;
use std::path::PathBuf;

mod diff;
mod pipfile_lock;

use crate::diff::{print_diff, Diff};
use crate::pipfile_lock::PipfileLock;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(
        help = "Path to Pipfile.lock. If omitted, assumes Pipfile.lock in the current directory"
    )]
    pipfile_lock: Option<PathBuf>,

    #[arg(
        short = 'r',
        long,
        help = "Git reference to compare to. Defaults to HEAD"
    )]
    git_ref: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pipfile_path =
        std::fs::canonicalize(args.pipfile_lock.unwrap_or_else(|| "Pipfile.lock".into()))?;

    let file = std::fs::File::open(&pipfile_path)?;

    let lockfile = PipfileLock::from_reader(file)?;

    let git_ref = args.git_ref.unwrap_or_else(|| "HEAD".to_owned());

    let repo = git2::Repository::discover(&pipfile_path.parent().unwrap())?;
    let path_in_repo: PathBuf = pipfile_path
        .strip_prefix(&repo.path().parent().unwrap())?
        .into();

    let obj = repo
        .resolve_reference_from_short_name(&git_ref)?
        .peel_to_tree()?
        .get_path(&path_in_repo)?
        .to_object(&repo)?;

    let blob = obj.as_blob().unwrap();

    let old_lockfile = pipfile_lock::PipfileLock::from_slice(blob.content())?;

    let diff = Diff::compare_dependencies(&lockfile.default, &old_lockfile.default);
    let diff_develop = Diff::compare_dependencies(&lockfile.develop, &old_lockfile.develop);

    println!("Default:");
    print_diff(&diff);

    println!();
    println!("Development:");
    print_diff(&diff_develop);

    Ok(())
}
