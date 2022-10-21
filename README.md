# YXY &emsp; [![crates badge]][crates.io] [![docs badge]][docs.rs] [![ci badge]][ci] [![build badge]][build]

[crates badge]: https://img.shields.io/crates/v/yxy.svg?logo=rust
[crates.io]: https://crates.io/crates/yxy
[docs badge]: https://img.shields.io/docsrs/yxy/latest?label=docs.rs&logo=docs.rs
[docs.rs]: https://docs.rs/yxy
[ci badge]: https://github.com/DumpTime/yxy/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/DumpTime/yxy/actions/workflows/ci.yml 
[build badge]: https://github.com/DumpTime/yxy/actions/workflows/build.yml/badge.svg
[build]: https://github.com/DumpTime/yxy/actions/workflows/build.yml

YXY(YSchool) platform API bindings, written in rust.

---

## Features

- [X] Query electricity balance.
- [X] Simulate APP login.
- [X] Subscribe some balance status. ([`yxy-cli`][yxy-cli])
  - WeChat(ServerChan)
- [X] Create electricity recharge transaction.

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

## CLI

You can use a simple tool: [`yxy-cli`][yxy-cli]

## For other programming languages

Also you can try [`yxy-abi`][yxy-abi]

## Disclaimer

For learning only, do not use for commercial purposes.

[yxy-cli]: https://github.com/DumpTime/yxy/tree/dev/crates/cli
[yxy-abi]: https://github.com/DumpTime/yxy/tree/dev/crates/abi
