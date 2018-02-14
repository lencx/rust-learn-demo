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

* [Hello World](./hello_world)
* [Guessing Game](./guessing_game)

<details>
<summary>Common Programming Concepts</summary>

* [variables](./common_concepts/variables)
* [data_types](./common_concepts/data_types)
* [control flow - branches](./common_concepts/control_flow/control_flow_branches)
* [control flow - loop](./common_concepts/control_flow/control_flow_loop)

</details>

<details>
<summary>Ownership</summary>

* [ownership - what is ownership](./ownership/ownership_what)
* [ownership - references & borrowing](./ownership/ownership_references_borrowing)
* [ownership - slice](./ownership/ownership_slices)

</details>

<details>
<summary>Structs</summary>

* [Defining Structs](./structs/defining_structs)
* [Example - Using Structs](./structs/rectangles)
* [Method Syntax](./structs/method_syntax)

</details>