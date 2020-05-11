![wip](https://img.shields.io/badge/wip-work%20in%20progress-brightgreen)
[![made with Rust](https://img.shields.io/badge/made%20with-Rust-blue)](https://www.rust-lang.org/)
![Platform](https://img.shields.io/badge/platform-windows%20%7C%20linux-informational)
[![GitHub license](https://img.shields.io/github/license/jtommi/csv_converter)](https://github.com/jtommi/csv_converter/blob/master/LICENSE)

CSV Converter
==============================================

This tool helps you convert CSV files from one format to another.

It is written in Rust.

# Usage
Currently the tool can only be used through the command line.  
You have two specify the input file as argument 1 and the output filename as argument 2.  
e.g. `csv_converter input.csv outpurt.csv`

# Development
## VS Code
This project was set up in Visual Code Studio.

It contains a devcontainer set up to start coding within minutes.  
There are also tasks defined to build, run and build for release

If you don't feel like using VS Code, you can of course compile and run the code in the standard way.
## Manual set up
1. Follow the [official guide](https://www.rust-lang.org/tools/install) to install Rust
2. Open a terminal
3. Build or run the code
   * Build: `cargo build` 
   * Run: `cargo run`
   * Build optimized version for release: `cargo build --release`