use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

use regex::Regex;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ParseVersionError {
    #[error("Non-PEP-440 version")]
    InvalidFormat,
    #[error("Can't parse number: {0}")]
    ParseNumber(#[from] ParseIntError),
    #[error("Invalid pre-release identifier")]
    InvalidPreRelease,
}

/// Pre-release cycle according to [PEP-440](https://peps.python.org/pep-0440/).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreReleaseCycle {
    Alpha,
    Beta,
    ReleaseCandidate,
}

impl FromStr for PreReleaseCycle {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "alpha" => Ok(Self::Alpha),
            "b" | "beta" => Ok(Self::Beta),
            "rc" | "c" | "pre" | "preview" => Ok(Self::ReleaseCandidate),
            _ => Err(ParseVersionError::InvalidPreRelease),
        }
    }
}

/// A [PEP-440](https://peps.python.org/pep-0440/) version
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    epoch: Option<u32>,
    segments: Vec<u32>,
    pre_release: Option<(PreReleaseCycle, u32)>,
    post_release: Option<u32>,
    dev_release: Option<u32>,
}

impl Version {
    /// Optional version-epoch
    pub fn epoch(&self) -> Option<u32> {
        self.epoch
    }

    /// Version segments, including the major version.
    pub fn segments(&self) -> &Vec<u32> {
        &self.segments
    }

    /// Major version
    pub fn major(&self) -> u32 {
        // a major version should always be present, or it's a bug
        self.segments[0]
    }

    /// Optional pre-release version
    pub fn pre_release(&self) -> Option<(PreReleaseCycle, u32)> {
        self.pre_release
    }

    /// Optional post-release version
    pub fn post_release(&self) -> Option<u32> {
        self.post_release
    }

    /// Optional dev-release version
    pub fn dev_release(&self) -> Option<u32> {
        self.dev_release
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?x)
            v?
            (?:(?P<epoch>\d+)!)?
            (?P<major>\d+)(?P<segments>(?:\.\d+)*)
            (?:(?P<precycle>[a-z]+)(?P<pre>\d+)?)?
            (?:(\.|-|_)?(?:post|r|rev)(?P<post>\d+))?
            (?:(\.|-|_)?dev(?P<dev>\d+))?",
        )
        .unwrap();

        if let Some(captures) = re.captures(s.trim()) {
            let epoch = captures
                .name("epoch")
                .map(|m| m.as_str().parse())
                .transpose()?;
            let mut segments = Vec::with_capacity(4); // 4 segments should do for 99.9% of packages
            segments.push(
                captures
                    .name("major")
                    .map(|m| m.as_str().parse())
                    .transpose()?
                    .ok_or(ParseVersionError::InvalidFormat)?,
            );

            if let Some(minor_segments) = captures.name("segments") {
                let mut minors = minor_segments
                    .as_str()
                    .split('.')
                    .skip(1) // minor_segments start with a dot, skip first empty element
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()?;
                segments.append(&mut minors);
            }
            let pre_release = captures
                .name("precycle")
                .map(|m| m.as_str().parse())
                .transpose()?
                .zip(Some(
                    captures
                        .name("pre")
                        .map(|m| m.as_str().parse())
                        .transpose()?
                        .unwrap_or(0),
                ));
            let post_release = captures
                .name("post")
                .map(|m| m.as_str().parse())
                .transpose()?;
            let dev_release = captures
                .name("dev")
                .map(|m| m.as_str().parse())
                .transpose()?;

            Ok(Self {
                epoch,
                segments,
                pre_release,
                post_release,
                dev_release,
            })
        } else {
            Err(ParseVersionError::InvalidFormat)
        }
    }
}

impl std::cmp::PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.epoch
            .cmp(&other.epoch)
            .then(self.segments.cmp(&other.segments))
            .then(self.post_release.cmp(&other.post_release))
            .then(match (&self.pre_release, &other.pre_release) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(pre_self), Some(pre_other)) => pre_self.cmp(&pre_other),
            })
            .then(match (&self.dev_release, &other.dev_release) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(dev_self), Some(dev_other)) => dev_self.cmp(&dev_other),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(
            "1.0.0".parse(),
            Ok(Version {
                epoch: None,
                segments: vec![1, 0, 0],
                pre_release: None,
                post_release: None,
                dev_release: None
            })
        );

        assert_eq!(
            "0.1.0".parse(),
            Ok(Version {
                epoch: None,
                segments: vec![0, 1, 0],
                pre_release: None,
                post_release: None,
                dev_release: None
            })
        );

        assert_eq!(
            "1".parse(),
            Ok(Version {
                epoch: None,
                segments: vec![1],
                pre_release: None,
                post_release: None,
                dev_release: None
            })
        );

        let complete_version = "1!2.3.4.9rc1.post5.dev6".parse::<Version>().unwrap();
        assert_eq!(complete_version.epoch(), Some(1));
        assert_eq!(complete_version.major(), 2);
        assert_eq!(complete_version.segments(), &vec![2, 3, 4, 9]);
        assert_eq!(
            complete_version.pre_release(),
            Some((PreReleaseCycle::ReleaseCandidate, 1))
        );
        assert_eq!(complete_version.post_release(), Some(5));
        assert_eq!(complete_version.dev_release(), Some(6));
        assert_eq!(
            complete_version,
            Version {
                epoch: Some(1),
                segments: vec![2, 3, 4, 9],
                pre_release: Some((PreReleaseCycle::ReleaseCandidate, 1)),
                post_release: Some(5),
                dev_release: Some(6)
            }
        );

        assert_eq!(
            "1.0a3".parse::<Version>(),
            Ok(Version {
                epoch: None,
                segments: vec![1, 0],
                pre_release: Some((PreReleaseCycle::Alpha, 3)),
                post_release: None,
                dev_release: None
            })
        );

        assert_eq!(
            "1.0a".parse::<Version>(),
            Ok(Version {
                epoch: None,
                segments: vec![1, 0],
                pre_release: Some((PreReleaseCycle::Alpha, 0)),
                post_release: None,
                dev_release: None
            })
        );

        assert_eq!(
            "1.0b10".parse::<Version>(),
            Ok(Version {
                epoch: None,
                segments: vec![1, 0],
                pre_release: Some((PreReleaseCycle::Beta, 10)),
                post_release: None,
                dev_release: None,
            })
        );

        assert_eq!(
            "1.0d3".parse::<Version>(),
            Err(ParseVersionError::InvalidPreRelease)
        );
        assert_eq!(
            "a".parse::<Version>(),
            Err(ParseVersionError::InvalidFormat)
        );
    }

    #[test]
    fn test_version_ordering() {
        fn cmp(a: &str, b: &str) -> Ordering {
            let a: Version = a.parse().unwrap();
            let b: Version = b.parse().unwrap();

            a.cmp(&b)
        }

        assert_eq!(cmp("1!1.0", "2.0"), Ordering::Greater);
        assert_eq!(cmp("2!1.0", "1!2.0"), Ordering::Greater);

        assert_eq!(cmp("1.0.0", "0.1.0"), Ordering::Greater);
        assert_eq!(cmp("1.1.0", "1.0.0"), Ordering::Greater);
        assert_eq!(cmp("1.1.0", "1.0.1"), Ordering::Greater);
        assert_eq!(cmp("1.0.1", "1.0.0"), Ordering::Greater);

        assert_eq!(cmp("1.0.0", "1.0.0a1"), Ordering::Greater);
        assert_eq!(cmp("1.0.0b1", "1.0.0a1"), Ordering::Greater);
        assert_eq!(cmp("1.0.0rc1", "1.0.0b1"), Ordering::Greater);
        assert_eq!(cmp("1.0.0rc2", "1.0.0rc1"), Ordering::Greater);

        assert_eq!(cmp("1.0.0", "1.0.0.dev1"), Ordering::Greater);
        assert_eq!(cmp("1.0.0a1", "1.0.0a1.dev1"), Ordering::Greater);
        assert_eq!(cmp("1.0.0.dev1", "1.0.0a1.dev1"), Ordering::Greater);
        assert_eq!(cmp("1.0.0.dev2", "1.0.0.dev1"), Ordering::Greater);
    }
}
