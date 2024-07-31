## factors-rs

Example of Bitcoin contract for use with [Bitcoin Elftrace](https://github.com/halseth/elftrace).

**Contract input**: u32.

**Contract output**: 1 if input is non-trivial factor of 901, 0 otherwise.

## Usege
Compile:
```bash
cargo build --release
```

Resulting binary found in `target/riscv32i-unknown-none-elf/release/factors-rs`.
