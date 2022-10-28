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
