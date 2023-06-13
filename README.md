# `gsync`

![Actions](https://img.shields.io/github/actions/workflow/status/xavier2p/gitsync/ci.yml?label=Continuous%20Integration&logo=githubactions&style=for-the-badge)
![License](https://img.shields.io/github/license/xavier2p/gitsync?logo=github&style=for-the-badge)
![GitHub top language](https://img.shields.io/github/languages/top/xavier2p/gitsync?color=orange&logo=rust&style=for-the-badge)
![Documentation Status](https://img.shields.io/website?down_color=critical&down_message=DOWN&label=Documentation&logo=github&style=for-the-badge&up_color=success&up_message=UP&url=https%3A%2F%2Fxavier2p.github.io%2Fgitsync)
![Release](https://img.shields.io/github/v/release/xavier2p/gitsync?label=latest%20release&logo=github&style=for-the-badge)

_A tool to earn time at each `git` actions._

## Usage

```bash
gsync [OPTIONS] [MESSAGE]
```

## Arguments

* `<MESSAGE>`    The message of the commit

## Options

* `-h`, `--help`         Print help information
* `-l`, `--local`        If used, the commit won't be pushed
* `-t`, `--tag <TAG>`    The tag of the commit
* `-u`, `--updated`      Add only the updated files
* `-v`, `--verbose`      Enable verbose mode
* `-V`, `--version`      Print version information

## Examples

```bash
gsync "My commit message"
gsync -uvlt v1.0.0 "My commit message"
```

## Installation

```bash
git clone <REPOSITORY_URL>
cd gitsync
cargo build --release
```

## Add to PATH

```bash
sudo cp target/release/gsync /usr/local/bin
```

## License

This project is under the MIT License.

## Author

**gsync** © [Xavier2p](https://github.com/Xavier2p)
