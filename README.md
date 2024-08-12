# touch-rs

[![madewith](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

Make a file like the Linux `touch` command and even truncate it to a specified size, either with bytes or human-readable sizes like `1M` or `26000k`.

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
Usage: touch-rs <FILE> [SIZE]

Arguments:
  <FILE>  Path to file
  [SIZE]  Size in bytes or human-readable format (e.g., 1M, 2G) [default: ]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Benchmarking

With `touch-rs` and [hyperfine](https://github.com/sharkdp/hyperfine) installed you can do:

```bash
$ hyperfine --warmup 3 "touch-rs test 12345 && rm test"
Benchmark 1: touch-rs test 12345 && rm test
  Time (mean ± σ):      13.7 ms ±   1.5 ms    [User: 3.0 ms, System: 11.3 ms]
  Range (min … max):    11.7 ms …  22.5 ms    151 runs
```

If you are on Linux you can benchmark against `touch`:

```bash
$ hyperfine --warmup 3 "touch-rs test && rm test" "touch test && rm test"
Benchmark 1: touch-rs test && rm test
  Time (mean ± σ):       3.6 ms ±   0.4 ms    [User: 1.3 ms, System: 0.1 ms]
  Range (min … max):     3.1 ms …   6.5 ms    638 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Benchmark 2: touch test && rm test
  Time (mean ± σ):       3.8 ms ±   1.9 ms    [User: 1.2 ms, System: 0.1 ms]
  Range (min … max):     3.2 ms …  52.8 ms    661 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Summary
  touch-rs test && rm test ran
    1.04 ± 0.55 times faster than touch test && rm test
```

## License

This project is licensed under the terms of the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html), see [LICENSE](LICENSE) for details.
