[![crates.io](https://img.shields.io/crates/v/is_affected)](https://crates.io/crates/is_affected)
[![Pipeline Status](https://gitlab.com/DeveloperC/is_affected/badges/main/pipeline.svg)](https://gitlab.com/DeveloperC/is_affected/-/commits/main)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A utility for checking and listing the affected resources across a range of commits, useful when working with monorepos.


## Content
 * [Usage](#usage)
   + [Usage - Git Environment Variables](#usage-git-environment-variables)
   + [Usage - Logging](#usage-logging)
 * [CICD Examples](#cicd-examples)
  + [GitLab CI Rust Project Example](#gitlab-ci-rust-project-example)
    + [Via Cargo](#via-cargo)
    + [Via Binary Download](#via-binary-download)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [End-to-End Testing](#end-to-end-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
Is Affected operates upon a range of Git commits in the repositories' history.
To specify the range of commits you can use either the `--from-commit-hash <commit-hash>` or `--from-reference <reference>` arguments.
The range of commits starts exclusively from the commit specified till inclusively of `HEAD`.
One of these from arguments are required, but they conflict and can not be used together.

Over the range of commits you can perform two possible operations.
Either you can list all the affected resources by supplying the `--list` flag.
Or you can check that specific resources have been affected, through the `--affects-current-directory` flag or the `--affects <resources>` argument.
With the `--affects-current-directory` flag the affected resources are checked if they are within the current directory or sub-directories.
Using the `--affects <resource>` argument you can supply multiple regexes and it is checked if any of the affected resources match any.
If either of the affects checks are met then Is Affected return a zero status code, otherwise it return a non-zero status code.
One of the output arguments are required, but they conflict and can not be used together.


### Usage - Git Environment Variables
When looking for a repository the Git environment variables are respected.
When `${GIT_DIR}` is set, it takes precedence and Is affected begins searching for a repository in the directory specified in `${GIT_DIR}`.
When `${GIT_DIR}` is not set, Is Affected searches for a repository beginning in the current directory.


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## CICD Examples
### GitLab CI Rust Project Example
#### Via Cargo
See [Compiling via Cargo](#compiling-via-cargo) for more details about installing via Cargo.

__Note - This example downloads the latest `0.*` version.__

```
example-stage:
  stage: example-stage
  image: rust
  before_script:
    - cargo install is_affected --version ^0
  script:
    - cd monorepo/
    # Check that the directory 'monorepo/' is affected in the merge request or else skip the stage.
    - /usr/local/cargo/bin/is_affected --from-reference "origin/${CI_MERGE_REQUEST_TARGET_BRANCH_NAME}" --affects-current-directory || exit 0
    - ... rest of the stage
  rules:
    - if: $CI_MERGE_REQUEST_ID
```


#### Via Binary Download
See [Downloading Binary](#downloading-binary) for more details about Binary downloads.

__Note - This example downloads version `0.4.1`.__
```
example-stage:
  stage: example-stage
  image: rust
  before_script:
    - wget -q -O tmp.zip "https://gitlab.com/DeveloperC/is_affected/-/jobs/artifacts/0.4.1/download?job=release-binary-compiling-x86_64-linux-musl" && unzip tmp.zip && rm tmp.zip
    - is_affected="$(pwd)/is_affected"
  script:
    - cd monorepo/
    # Check that the directory 'monorepo/' is affected in the merge request or else skip the stage.
    - ${is_affected} --from-reference "origin/${CI_MERGE_REQUEST_TARGET_BRANCH_NAME}" --affects-current-directory || exit 0
    - ... rest of the stage
  rules:
    - if: $CI_MERGE_REQUEST_ID
```


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/is_affected/-/releases](https://gitlab.com/DeveloperC/is_affected/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/is_affected.git
cd is_affected/
cargo build --release
```

The compiled binary is present at `target/release/is_affected`.


## Compiling via Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/is_affected) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/is_affected`.

```
cargo install is_affected
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```
cargo install is_affected --version 0.4.1
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```
cargo install is_affected --version ^0
```

Will download the latest `0.*` release whether that is `0.4.1` or `0.7.0`.


## Unit Testing
The unit test suite has several parameterised tests, Cargo is used to set up and run all the unit tests.

```
cargo test
```

## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End behaviour driven test suite using the behave framework (https://github.com/behave/behave).
To run the test suite you need to first build a binary, install Python3, install behave and then execute behave to run the behaviour driven test suite.

__Note - You can't use --release as the test suite uses `target/debug/is_affected`.__

```
cargo build
cd is_affected/end-to-end-tests/
virtualenv -p python3 .venv
source .venv/bin/activate
pip3 install -r requirements.txt
behave
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://gitlab.com/DeveloperC/is_affected/-/issues](https://gitlab.com/DeveloperC/is_affected/-/issues).
