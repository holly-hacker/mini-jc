# `mini-jc`

A [jc](https://github.com/kellyjonbrazil/jc) clone with reasonable performance.

## Motivation

I use `jc` to show system stats in my taskbar on my Linux setup. Since `jc` is written in Python, it took ~50ms to spin up, parse the 3 lines of output from `free` and output it as json. I found this too slow as I have to run this every second and I don't want to give up a permanent 5% of a CPU core just to show my memory usage.

Also I was bored.

## Installation

### Using a Rust toolchain

Run `cargo install --git https://github.com/holly-hacker/mini-jc`.

<!-- TODO: add nix flake -->

## Differences from `jc`

- `mini-jc` is a single, statically linked executable so it is fairly to install
- `mini-jc` executes magnitudes faster due to not having to start up a runtime
- `mini-jc` supports way fewer parsers. I will likely only add support for tools I use myself.
- `mini-jc` will convert "human-readable" sizes to absolute values
- `mini-jc` tries to copy `jc`'s output naming, but doesn't do so everywhere

## Supported parsers

- `free`: matches `jc`
- `df`: matches `jc`, property names have being changed to be more consistent

## Performance

<!--
cargo build --release

hyperfine -N --warmup 100 --input test-cases/free/no-args.txt "jc --free" "target/release/mini-jc free" --export-markdown out.md && cat out.md && rm out.md

-->

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `jc --free` | 44.3 ± 0.3 | 43.7 | 45.5 | 112.19 ± 9.90 |
| `mini-jc free` | 0.4 ± 0.0 | 0.3 | 1.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `jc --df` | 46.8 ± 0.8 | 46.0 | 50.8 | 111.48 ± 10.33 |
| `mini-jc df` | 0.4 ± 0.0 | 0.3 | 0.7 | 1.00 |
