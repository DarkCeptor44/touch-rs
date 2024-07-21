# touch-rs

[![madewith](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

Make a file like the Linux `touch` command and even truncate it to a specified size. Made mainly for Windows but it works on any Unix-like system.

## Installation

### Cargo

```bash
cargo install --git https://github.com/DarkCeptor44/touch-rs.git
```

### From source

```bash
git clone https://github.com/DarkCeptor44/touch-rs.git
cd touch-rs
cargo install --path .
```

## Usage

```bash
$ touch-rs -h
Usage: touch-rs [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to file

Options:
  -s, --size <SIZE>  [default: 0]
  -h, --help         Print help
  -V, --version      Print version
```

## Benchmarking

With `touch-rs` and [hyperfine](https://github.com/sharkdp/hyperfine) installed you can do:

```bash
$ hyperfine --warmup 3 "touch-rs test-rs -s 12345&&rm test-rs"
Benchmark 1: touch-rs test-rs -s 12345&&rm test-rs
  Time (mean ± σ):      12.3 ms ±   1.6 ms    [User: 2.9 ms, System: 9.6 ms]
  Range (min … max):    10.1 ms …  24.1 ms    164 runs
```

## License

This project is licensed under the terms of the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html), see [LICENSE](LICENSE) for details.
