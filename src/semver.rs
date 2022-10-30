use std::{num::ParseIntError, str::FromStr};

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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreReleaseCycle {
    Alpha,
    Beta,
    ReleaseCandidate,
}

impl FromStr for PreReleaseCycle {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::Alpha),
            "b" => Ok(Self::Beta),
            "rc" => Ok(Self::ReleaseCandidate),
            _ => Err(ParseVersionError::InvalidPreRelease),
        }
    }
}

/// [PEP-440](https://peps.python.org/pep-0440/) version
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    epoch: Option<u32>,
    segments: Vec<u32>,
    pre_release: Option<(PreReleaseCycle, u32)>,
    post_release: Option<u32>,
    dev_release: Option<u32>,
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?x)
            (?:(?P<epoch>\d+)!)?
            (?P<major>\d+)(?P<segments>(?:\.\d+)*)
            (?:(?P<precycle>[a-z]+)(?P<pre>\d+))?
            (?:\.post(?P<post>\d+))?
            (?:\.dev(?P<dev>\d+))?",
        )
        .unwrap();

        if let Some(captures) = re.captures(s) {
            let epoch = captures
                .name("epoch")
                .map(|m| m.as_str().parse())
                .transpose()?;
            let mut segments = vec![captures
                .name("major")
                .map(|m| m.as_str().parse())
                .transpose()?
                .ok_or(ParseVersionError::InvalidFormat)?];

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
                .zip(
                    captures
                        .name("pre")
                        .map(|m| m.as_str().parse())
                        .transpose()?,
                );
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_is_major_update() {
    //     assert!(is_major_update("1.0.0", "2.0.0"));
    //     assert!(!is_major_update("1.0.0", "1.0.0"));
    //     assert!(!is_major_update("1.0.0", "1.0.1"));
    //     assert!(!is_major_update("1.0.0", "1.1.0"));

    //     assert!(is_major_update("1", "2"));
    //     assert!(!is_major_update("1", "1"));
    //     assert!(is_major_update("9", "10"));

    //     assert!(!is_major_update("1.0", "2.0.0"));
    //     assert!(!is_major_update("1.0.3", "2.0"));
    // }

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
            "1".parse(),
            Ok(Version {
                epoch: None,
                segments: vec![1],
                pre_release: None,
                post_release: None,
                dev_release: None
            })
        );

        assert_eq!(
            "1!2.3.4.9rc1.post5.dev6".parse(),
            Ok(Version {
                epoch: Some(1),
                segments: vec![2, 3, 4, 9],
                pre_release: Some((PreReleaseCycle::ReleaseCandidate, 1)),
                post_release: Some(5),
                dev_release: Some(6)
            })
        );

        assert_eq!(
            "1.0c3".parse::<Version>(),
            Err(ParseVersionError::InvalidPreRelease)
        );
        assert_eq!(
            "a".parse::<Version>(),
            Err(ParseVersionError::InvalidFormat)
        );
    }
}
