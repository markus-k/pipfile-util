# `pipfile-util`
[![CI](https://github.com/markus-k/pipfile-util/actions/workflows/rust.yml/badge.svg)](https://github.com/markus-k/pipfile-util/actions/workflows/rust.yml)
![License](https://img.shields.io/github/license/markus-k/s3-proxy)
[![Crates.io](https://img.shields.io/crates/v/pipfile-util)](https://crates.io/crates/pipfile-util)

A command line utility for working with pipenv's Pipfile's.

Current features:
 * List changed versions of a freshly locked `Pipfile.lock` against it's latest Git version

`pipfile-util` is still in early development and not feature-complete. Breaking changes are expected before version 1.0.

## Usage

To compare a freshly locked `Pipfile.lock` to the latest committed one, run

```
$ pipfile-util diff path/to/Pipfile.lock 

Default:
Changed:
  certifi: 2022.5.18.1 => 2022.9.24
  lxml: 4.9.0 => 4.9.1
  tinycss2: 1.1.1 => 1.2.1

Development:
Changed:
  pylint: 2.12.2 => 2.15.5
New:
  tomlkit: 0.11.6
  tomli: 2.0.1
Deleted:
  setuptools: 62.3.2
  toml: 0.10.2
```

The output from `pipfile-util diff` can also be easily used to create commit messages:

```sh
# lock your Pipfile to install updates
pipenv lock
git add Pipfile.lock
# create a commit, with the output from pipfile-util as a template
git commit -t <(pipfile-util diff)

# or for fish-shell (and others not supporting <(..) syntax):
git commit -t (pipfile-util diff | psub)
```

For more information, run `pipfile-util --help`.

## Installation

If you have Rust installed on your machine, you can install `pipfile-util` with Cargo:

```sh
cargo install pipfile-util
```

## License

`pipfile-util` is licensed under the Apache-2.0 license.
