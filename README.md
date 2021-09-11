[![crates.io](https://img.shields.io/crates/v/is_effected)](https://crates.io/crates/is_effected) [![pipeline status](https://gitlab.com/DeveloperC/is_effected/badges/main/pipeline.svg)](https://gitlab.com/DeveloperC/is_effected/-/commits/main) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A utility to check if a particular file/directory has been effected within a range of commits. Useful for monorepos to check sub-repositories.


## Content
 * [Usage](#usage)
   + [Usage - Git Environment Variables](#usage-git-environment-variables)
   + [Usage - Logging](#usage-logging)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
Is Effected operates upon a range of Git commits in the repositories' history.
To specify the range of commits you can use either the `--from-commit-hash <commit-hash>` or `--from-reference <reference>` arguments.
The range of commits starts exclusively from the commit specified till inclusively of `HEAD`.
The from arguments can not be used together but one is required.

Over the range of commits you can either specify all the effected resources to be listed out via the `--list` argument.
Otherwise you can provide regexes via the `--effects <resource>` argument and if any of the effected resources within the range of commits match, then Is Effected return a zero status code otherwise it return a non-zero status code.
Either of these output arguments are required and can not be used together.

### Usage - Git Environment Variables
When looking for a repository the Git environment variables are respected.
When `$GIT_DIR` is set, it takes precedence and Is Effected begins searching for a repository in the directory specified in `$GIT_DIR`.
When `$GIT_DIR` is not set, Is Effected searches for a repository beginning in the current directory.


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/is_effected/-/releases](https://gitlab.com/DeveloperC/is_effected/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/is_effected.git
cd is_effected/
cargo build --release
```

The compiled binary is present at `target/release/is_effected`.


## Compiling via Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/is_effected) and then compiles the binary locally, placing the compiled binary at `$HOME/.cargo/bin/is_effected`.

```
cargo install is_effected
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```
cargo install is_effected --version 0.2.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```
cargo install is_effected --version ^0
```

Will download the latest `0.*` release whether that is `0.2.2` or `0.7.0`.


## Unit Testing
The unit test suite has several parameterised tests, Cargo is used to set up and run all the unit tests.

```
cargo test
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://gitlab.com/DeveloperC/is_effected/-/issues](https://gitlab.com/DeveloperC/is_effected/-/issues).
