use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

mod pipfile_lock {
    use std::{collections::HashMap, fmt::Display, io::Read};

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Meta {
        #[serde(rename = "pipfile-spec")]
        pub pipfile_spec: i32,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
    #[serde(untagged)]
    pub enum Dependency {
        Git {
            #[serde(rename = "ref")]
            git_ref: String,
        },
        Pip {
            version: String,
        },
    }

    impl Display for Dependency {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Git { git_ref } => write!(f, "{git_ref}"),
                Self::Pip { version } => write!(f, "{}", version.trim_start_matches("==")),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct PipfileLock {
        #[serde(rename = "_meta")]
        pub meta: Meta,
        pub default: HashMap<String, Dependency>,
        pub develop: HashMap<String, Dependency>,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Incompatble Pipfile.lock spec: {0}")]
        IncompatiblePipfileLockSpec(i32),
        #[error("Deserialize error: {0}")]
        Deserialize(#[from] serde_json::Error),
    }

    impl PipfileLock {
        fn validate(self) -> Result<Self, Error> {
            if self.meta.pipfile_spec != 6 {
                return Err(Error::IncompatiblePipfileLockSpec(self.meta.pipfile_spec));
            }

            Ok(self)
        }

        pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
            let pipfile: PipfileLock = serde_json::from_reader(reader)?;

            pipfile.validate()
        }

        pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
            let pipfile: PipfileLock = serde_json::from_slice(slice)?;

            pipfile.validate()
        }
    }
}

use pipfile_lock::Dependency;

#[derive(Debug, Default)]
pub struct Diff {
    pub changed: HashMap<String, (Dependency, Dependency)>,
    pub new: HashMap<String, Dependency>,
    pub deleted: HashMap<String, Dependency>,
}

pub fn compare_dependencies(
    new: &HashMap<String, Dependency>,
    old: &HashMap<String, Dependency>,
) -> Diff {
    let mut diff = Diff::default();

    for (dep_name, new_dep) in new.iter() {
        if let Some(old_dep) = old.get(dep_name) {
            if new_dep != old_dep {
                diff.changed
                    .insert(dep_name.clone(), (new_dep.clone(), old_dep.clone()));
            }
        } else {
            diff.new.insert(dep_name.clone(), new_dep.clone());
        }
    }

    for (dep_name, old_dep) in old.iter() {
        if new.get(dep_name).is_none() {
            diff.deleted.insert(dep_name.clone(), old_dep.clone());
        }
    }

    diff
}

fn print_diff(diff: &Diff) {
    if !diff.changed.is_empty() {
        println!("Changed:");
        for (dep_name, (new, old)) in diff.changed.iter() {
            println!("  {dep_name}: {} => {}", old, new);
        }
    }

    if !diff.new.is_empty() {
        println!("New:");
        for (dep_name, new) in diff.new.iter() {
            println!("  {dep_name}: {}", new);
        }
    }

    if !diff.deleted.is_empty() {
        println!("Deleted:");
        for (dep_name, deleted) in diff.deleted.iter() {
            println!("  {dep_name}: {}", deleted);
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    pipfile: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pipfile_path = args.pipfile.unwrap_or_else(|| "Pipfile.lock".into());

    let file = std::fs::File::open(&pipfile_path)?;

    let lockfile = pipfile_lock::PipfileLock::from_reader(file)?;

    let repo = git2::Repository::discover(&pipfile_path.parent().unwrap())?;
    println!("Repo path: {:?}", repo.path());
    println!("Pipfile path: {:?}", pipfile_path);

    let path_in_repo: PathBuf = pipfile_path
        .strip_prefix(&repo.path().parent().unwrap())?
        .into();

    println!("Pipfile path in repo: {:?}", path_in_repo);

    let obj = repo
        .find_reference("HEAD")?
        .peel_to_tree()?
        .get_path(&path_in_repo)?
        .to_object(&repo)?;

    let blob = obj.as_blob().unwrap();

    let old_lockfile = pipfile_lock::PipfileLock::from_slice(blob.content())?;

    let diff = compare_dependencies(&lockfile.default, &old_lockfile.default);

    print_diff(&diff);

    Ok(())
}
