# Miniproject 8 For IDS 706

This repo contains 3 Rust scripts that do the following:

## Rust Data Processing

This repository contains a collection of Rust scripts for data extraction, transformation, loading, and querying. The scripts interact with a SQLite database and a CSV file.

### Scripts

#### 1. `extract.rs`

This script downloads a CSV file from a specified URL and saves it to a local directory. It uses the `reqwest` crate for HTTP requests and the `tokio` crate for asynchronous programming.

To run this script, use the following command:

```bash
cargo run --bin extract
```

#### 2. transform_load.rs
This script reads data from a CSV file and loads it into a SQLite database. It uses the csv crate to read CSV files and the rusqlite crate to interact with SQLite databases.

To run this script, use the following command:

```
cargo run --bin transform_load
```

#### 3. query.rs
This script queries the SQLite database and prints the cars with the most horsepower. It uses the rusqlite crate to interact with SQLite databases.

To run this script, use the following command:

```
cargo run --bin query
```

### Dependencies
This project uses several dependencies, including openssl, reqwest, tokio, csv, and rusqlite. Make sure to add these to your Cargo.toml file.

### Setup
Before running the scripts, make sure to install Rust and Cargo on your system. You can download them from the official Rust website.

Then, you can run the scripts as described above.

## Environment Details:

This is a basic environment that comes with the following software choices preinstalled:

### System Tools

- [curl/curl](https://github.com/curl/curl): the command line tool for transferring data over a metric boatload of protocols.
- git: the Git SCM tool.
- [gnupg2](https://gnupg.org/): a complete and free implementatiuon of the OpenPGP standard.
- [stedolan/jq](https://github.com/stedolan/jq) - a command line JSON parser.
- [sudo](https://www.sudo.ws/) - the superuser authority delegation tool.
- [zsh](https://www.zsh.org/) - interactive terminal (alternative to `bash`).
- [ohmyzsh/ohmyzsh](https://github.com/ohmyzsh/ohmyzsh) - a delightful community driven framework for managing zsh config.
- [vim](https://www.vim.org/) - a text editor
- build essentials - tools for compiling and linking code
- [openssl](https://www.openssl.org/) - tls and ssl toolkit

### Rust Tools

Besides Rust and Cargo, the image comes with the following Rust related tooling:

- [rustup](https://rustup.rs/): installer and toolchain manager
- [rustfmt](https://github.com/rust-lang/rustfmt): a tool for formatting Rust code according to style guidelines
- [clippy](https://github.com/rust-lang/rust-clippy): lints to catch common mistakes and improve your Rust code

### VS Code Extensions

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer): an alternative rust language server to the RLS.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb): native debugger based on LLDB.
- [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates): helps Rust developers managing dependencies with Cargo.toml.
- [Live Share](https://marketplace.visualstudio.com/items?itemName=ms-vsliveshare.vsliveshare): collaborative, multi-user remote editing from directly within the editor.

### Operating System

- [Ubuntu 18.04](https://releases.ubuntu.com/18.04.4/): The 18.04 LTS version of Ubuntu.