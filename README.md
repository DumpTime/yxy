# YXY &emsp; [![ci badge]][ci] [![build badge]][build]

[crates badge]: https://img.shields.io/crates/v/yxy.svg?logo=rust
[crates.io]: https://crates.io/crates/yxy
[docs badge]: https://img.shields.io/docsrs/yxy/latest?label=docs.rs&logo=docs.rs
[docs.rs]: https://docs.rs/yxy
[ci badge]: https://github.com/DumpTime/yxy/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/DumpTime/yxy/actions/workflows/ci.yml 
[build badge]: https://github.com/DumpTime/yxy/actions/workflows/build.yml/badge.svg
[build]: https://github.com/DumpTime/yxy/actions/workflows/build.yml

YXY(yiSchool) platform API bindings, written in rust.

---

## Features

- [X] Query electricity balance, usages/recharge records.
- [X] Simulate APP login.
- [X] Subscribe some balance status. ([`yxy-cli`][yxy-cli])
  - WeChat(ServerChan)
- [X] Create electricity recharge transaction.
- [ ] And more...(Waiting for your contribution)

## Development

1. Prepare `Rust` development environment.
2. Clone the repo

   ```bash
   git clone https://github.com/DumpTime/yxy.git
   ```
3. Build & test

   ```bash
   cargo test
   ```
4. Docs
   
   ```bash
   cargo doc --open
   ```
5. Check with `cargo-clippy`

   ```bash
   cargo clippy
   ```

## CLI

You can use a simple tool: [`yxy-cli`][yxy-cli]

## For other programming languages

Also you can try [`yxy-abi`][yxy-abi] or [`yxy-httpd`][yxy-httpd]

# Disclaimer
**A pure non-profit project, only for learning usage. Do not use for any commercial purposes.**
**Any inappropriate use is at your own risk.**

[yxy-cli]: https://github.com/DumpTime/yxy/tree/dev/crates/cli
[yxy-abi]: https://github.com/DumpTime/yxy/tree/dev/crates/abi
[yxy-httpd]: https://github.com/DumpTime/yxy/tree/dev/crates/httpd