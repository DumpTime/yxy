# YXY CLI &emsp; 

Command Line Interface for YXY

## Installation

1. Prepare `Rust` development environment.  
2. Build & Run.
   
   ```bash
   cargo run
   ```

## Features

> The following uses `yxy` to represent the main program

1. Query electricity by conf

    [conf.yaml.example][conf example]
   - `conf.yaml` file is located in the current working directory

     ```bash
     yxy-cli
     ```
   - Or in other place

     ```bash
     yxy-cli -c <PATH>
     ```
2. Other Queries

   1. UID

      > Get UID by simulating app login, so you need to register yxy app account first.
      >

      ```bash
      yxy-cli query uid <phone number>
      ```
   2. Electricity

      > (Simply query by UID without config file)
      >

      ```bash
      yxy-cli query ele <UID>
      ```

[crates badge]: https://img.shields.io/crates/v/yxy-cli.svg?logo=rust
[crates.io]: https://crates.io/crates/yxy-cli
[conf example]: conf.example.yaml
