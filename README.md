[![crates.io](https://img.shields.io/crates/v/is_affected)](https://crates.io/crates/is_affected)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

A utility for checking and listing the affected resources across a range of commits, useful when working with monorepos.

- [Usage](#usage)
- [Examples](#examples)
  - [GitHub Actions](#github-actions)
  - [GitLab CI](#gitlab-ci)
- [Installation](#installation)
  - [Binary](#binary)
  - [Cargo](#cargo)
  - [Docker](#docker)

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


## Examples
All CI examples set up the environment and then execute a script to perform the action.
This approach is recommended as it allows us to avoid provider-specific quirks and constraints, while also enabling us to execute the same logic locally.
It also enables the use of tools such as formatter and linters, ensuring consistency and reducing errors compared to embedding commands directly into the CI provider's custom syntax.

### GitHub Actions
```yaml
name: Example

on: pull_request

jobs:
  example:
    runs-on: ubuntu-latest
    container:
      image: rust
    steps:
      - name: Checkout code.
        uses: actions/checkout@v3
        # https://github.com/actions/checkout
        # Checks out only a single merge commit of the pull request and the target branch by default.
        with:
          ref: ${{ github.event.pull_request.head.sha }}
          fetch-depth: 0
      - name: Install is_affected.
        # Installation options
        # https://github.com/DeveloperC286/is_affected/tree/main/is_affected#downloading-binary
        # https://github.com/DeveloperC286/is_affected/tree/main/is_affected#compiling-via-cargo
        run: cargo install is_affected --version ^0
      - name: Example is_affected usage.
        run: ci/example.sh
```

```sh
#!/usr/bin/env sh

set -o errexit
set -o xtrace

# https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/variables
target="origin/${GITHUB_BASE_REF}"

# Only act if the directory 'frontend/' is affected in the merge/pull request.
if is_affected --from-reference "${target}" --affects "frontend/"; then
	echo "..."
fi
```


### GitLab CI
```yaml
example:
  image: rust
  before_script:
    # Installation options
    # https://github.com/DeveloperC286/is_affected/tree/main/is_affected#downloading-binary
    # https://github.com/DeveloperC286/is_affected/tree/main/is_affected#compiling-via-cargo
    - cargo install is_affected --version ^0
  script:
    - ci/example.sh
  rules:
    - if: $CI_MERGE_REQUEST_ID
```

```sh
#!/usr/bin/env sh

set -o errexit
set -o xtrace

# https://docs.gitlab.com/ee/ci/variables/predefined_variables.html
target="origin/${CI_MERGE_REQUEST_TARGET_BRANCH_NAME}"

# Only act if the directory 'frontend/' is affected in the merge/pull request.
if is_affected --from-reference "${target}" --affects "frontend/"; then
	echo "..."
fi
```


## Installation

### Binary
<!-- x-release-please-start-version -->
```sh
version="v0.6.0" && wget -O - "https://github.com/DeveloperC286/is_affected/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
```
<!-- x-release-please-end -->

Statically linked compiled binaries are available for download.
Visit the releases page at [https://github.com/DeveloperC286/is_affected/releases](https://github.com/DeveloperC286/is_affected/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.

### Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/is_affected) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/is_affected`.

```sh
cargo install is_affected
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

<!-- x-release-please-start-version -->
```sh
cargo install is_affected --version 0.6.0
```
<!-- x-release-please-end -->

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

<!-- x-release-please-start-version -->
```sh
cargo install is_affected --version ^0
```
<!-- x-release-please-end -->

Will download the latest `0.*` release whether that is `0.4.2` or `0.7.0`.

### Docker
You can use the Docker image published to [ghcr.io/developerc286/is_affected](https://github.com/DeveloperC286/is_affected/pkgs/container/is_affected).


## Issues/Feature Requests
Report issues or request features on our [GitHub Issues](https://github.com/DeveloperC286/is_affected/issues).
