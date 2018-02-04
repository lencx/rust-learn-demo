# Rust

> ahead-of-time compiled language

## Install & Uninstall

```bash
# install
curl https://sh.rustup.rs -sSf | sh

# uninstall
rustup self uninstall
```

## Compiling & Run

```bash
# compiling
rustc main.rs

# run
./main
```

## Cargo

```bash
[project]
    |-[src]
    |-[target]
    |   |-[debug]
    |   `-[release]
    |-Cargo.lock
    `-Cargo.tomal

```

### Config

> `Cargo.toml`

```toml
[package]

name = "package_name"
version = "version_number"
authors = ["your_name <your_email@example.com>"]
```

### Building & Running

```bash
# building
cargo build

# running
cargo run
```

### Building for Release

```bash
cargo build --release
```

### Making A New Cargo Project

```bash
cargo new project_name --bin
```

## Examples

* [hello_world](./hello_world)
* [guessing_game](./guessing_game)
* [variables](./variables)
* [data_types](./data_types)
* [functions](./functions)
