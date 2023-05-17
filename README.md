# YXY &emsp; [![ci badge]][ci] [![build badge]][build]

[crates badge]: https://img.shields.io/crates/v/yxy.svg?logo=rust
[crates.io]: https://crates.io/crates/yxy
[docs badge]: https://img.shields.io/docsrs/yxy/latest?label=docs.rs&logo=docs.rs
[docs.rs]: https://docs.rs/yxy
[ci badge]: https://github.com/DumpTime/yxy/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/DumpTime/yxy/actions/workflows/ci.yml 
[build badge]: https://github.com/DumpTime/yxy/actions/workflows/build.yml/badge.svg
[build]: https://github.com/DumpTime/yxy/actions/workflows/build.yml

YXY(yiSchool) platform HTTP API bindings, written in Rust.

---

## Features

- [X] Query electricity surplus, consumption and recharge records.
- [X] APP login stage simulation.
- [X] Subscribe some states. ([CLI][yxy-cli])
  - WeChat(ServerChan)
- [X] Create electricity recharge transactions.
- [ ] More... (Waiting for your contribution)

## Development

1. Prepare `Rust` development environment.
2. Clone the repo

   ```bash
   git clone https://github.com/DumpTime/yxy.git
   ```
3. Build
   
   ```bash
   cargo build
   ```
4. Docs
   
   ```bash
   cargo doc --open
   ```

## Command Line Interface

A simple tool: [`yxy-cli`][yxy-cli]

## For other programming languages

See more: [`yxy-abi`][yxy-abi] or [`yxy-httpd`][yxy-httpd]

# Disclaimer
Completely FREE software for learning only.
**Any inappropriate use is at your own risk.**

[yxy-cli]: https://github.com/DumpTime/yxy/tree/dev/crates/cli
[yxy-abi]: https://github.com/DumpTime/yxy/tree/dev/crates/abi
[yxy-httpd]: https://github.com/DumpTime/yxy/tree/dev/crates/httpd