<div align="center">
    <h1>
        <code>gsync</code>
    </h1>
    <div>
        <img src="https://img.shields.io/github/actions/workflow/status/xavier2p/gitsync/ci.yml?label=Continuous%20Integration&logo=githubactions&style=for-the-badge" />
        <img src="https://img.shields.io/github/languages/top/xavier2p/gitsync?color=orange&logo=rust&style=for-the-badge" />
    </div>
    <div>
        <img src="https://img.shields.io/github/license/xavier2p/gitsync?logo=github&style=for-the-badge" />
        <img src="https://img.shields.io/github/v/release/xavier2p/gitsync?label=latest%20release&logo=github&style=for-the-badge" />
        <img src="https://img.shields.io/website?down_color=critical&down_message=DOWN&label=Documentation&logo=github&style=for-the-badge&up_color=success&up_message=UP&url=https%3A%2F%2Fxavier2p.github.io%2Fgitsync" />
    </div>
</div>

---

> A tool to earn time at each `git` actions.

## Usage

```bash
gsync [OPTIONS] [MESSAGE]
```

## Arguments

* `<MESSAGE>`    The message of the commit

## Options

* `-h`, `--help`         Prints help information
* `-l`, `--local`        If used, the commit won't be pushed
* `-s`, `--sign`         Signs the commit
* `-t`, `--tag <TAG>`    Tags the commit
* `-u`, `--updated`      Adds only the updated files
* `-v`, `--verbose`      Enables verbose mode
* `-V`, `--version`      Prints version information

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
