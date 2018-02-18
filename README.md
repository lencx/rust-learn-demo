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

### Building & Running & Test

```bash
# cargo new project_name
# building
cargo build

# cargo new --bin project_name
# running
cargo run

cargo test
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

* [Variables](./common_concepts/variables)
* [Data Types](./common_concepts/data_types)
* [Control flow - branches](./common_concepts/control_flow/control_flow_branches)
* [Control flow - loop](./common_concepts/control_flow/control_flow_loop)

</details>

<details>
<summary>Ownership</summary>

* [What is ownership](./ownership/ownership_what)
* [References & Borrowing](./ownership/ownership_references_borrowing)
* [Slice](./ownership/ownership_slices)

</details>

<details>
<summary>Structs</summary>

* [Defining Structs](./structs/defining_structs)
* [Example - Using Structs](./structs/rectangles)
* [Method Syntax](./structs/method_syntax)

</details>

<details>
<summary>Enums</summary>

* [Defining Enums](./enums/defining_enum)
* [Match](./enums/control_flow_match)
* [Control Flow - `if let`](./enums/if_let)

</details>

<details>
<summary>Modules</summary>

* [mod & Filesystem](./modules/mod_filesystem)
* [Controlling Visibility with `pub`](./modules/visibility_pub)
* [mod Demo](./modules/mod_demo)
* [Referring to Names in Different Modules](./modules/nested_modules)

</details>

<details>
<summary>Common Collections</summary>

* [Vectors](./common_collections/vectors)

</details>