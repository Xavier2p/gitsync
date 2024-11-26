# `gsync`

[![ci-status](https://img.shields.io/github/actions/workflow/status/xavier2p/gitsync/ci.yml?label=Continuous%20Integration&logo=githubactions&style=for-the-badge)](https://github.com/Xavier2p/gitsync/actions)
[![rust](https://img.shields.io/github/languages/top/xavier2p/gitsync?color=orange&logo=rust&style=for-the-badge)](https://rust-lang.org)

[![license](https://img.shields.io/github/license/xavier2p/gitsync?logo=github&style=for-the-badge&color=yellow)](https://github.com/Xavier2p/gitsync/blob/main/LICENSE)
[![release](https://img.shields.io/github/v/release/xavier2p/gitsync?label=latest%20release&logo=github&style=for-the-badge)](https://github.com/Xavier2p/gitsync/releases)
[![docs-status](https://img.shields.io/website?down_color=critical&down_message=DOWN&label=Documentation&logo=github&style=for-the-badge&up_color=success&up_message=UP&url=https%3A%2F%2Fxavier2p.github.io%2Fgitsync)](https://xavier2p.github.io/gitsync)

---

> A tool to earn time at each `git` actions.

At each call, the tool performs:

```console
$ git add -A 
# or --updated if you use the `-u` option

$ git commit -m "<MESSAGE>"
# we can add the option `-s` to use commit signature

$ git tag -a <TAG_NAME> -m "<MESSAGE>"
# if you use `-t <NAME>`

$ git push
# only if you don't use `-l`, follows tags if selected
```

You can add `-v` to enable verbose at each step.

## Usage

```console
$ gsync help
A simple tool to use `git` with ease.
Written in Rust.

Usage: gsync [OPTIONS] <COMMAND>

Commands:
  build     Changes that affect the build system or external dependencies
  chore     Updating grunt tasks etc; no production code change
  ci        Changes on the CI/CD pipeline or other DevOps tools
  docs      Changes to the documentation
  feat      New feature for the user, not a new feature for build script
  fix       Bug fix for the user, not a fix to a build script
  perf      A new performance improvement
  refactor  Refactoring production code, eg. renaming a variable
  style     Formatting, missing semi colons, etc; no production code change
  test      Adding missing tests, refactoring tests; no production code change
  help      Print this message or the help of the given subcommand(s)

Options:
  -t, --tag <TAG>  Tag to assign at the commit
  -l, --local      If used, the commit won't be pushed, only committed
  -v, --verbose    Enable verbose mode
  -u, --updated    Add only the updated files
  -s, --sign       Sign the commit
  -h, --help       Print help
  -V, --version    Print version
```

## Examples

```bash
gsync "My commit message"
gsync -uvlt v1.0.0 "My commit message"
```

## Installation

```bash
git clone https://github.com/Xavier2p/gitsync.git && cd gitsync
cargo install --path .
```

## License

This project is under the MIT License.

## Author

**`gsync`** © [Xavier2p](https://github.com/Xavier2p)
