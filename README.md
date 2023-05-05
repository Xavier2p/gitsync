# `gsync`

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
