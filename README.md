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

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

[workspace]
member = [
    "crate_one",
    "crate_two"
]
```

> crate_one/Cargo.toml

```tomal
crate_two = {path = "../crate_two"}
```

> optimizations(default values)

|env|opt-level|
|:---:|:---:|
|dev|0|
|release|3|

### Building & Running & Test & Doc

```bash
# cargo new project_name
# building
cargo build

# cargo new --bin project_name
# running
cargo run

cargo test

cargo doc --open
```

### Building for Release

```bash
cargo build --release
```

### Making A New Cargo Project

```bash
cargo new project_name
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
* [Strings](./common_collections/strings)
* [Hash Maps](./common_collections/hash_maps)

</details>

<details>
<summary>Error handling</summary>

* [Unrecoverable Errors with `panic!`](./error_handling/unrecoverable_errors_panic)
* [Recoverable Errors with `Result`](./error_handling/recoverable_errors/)
* [To `panic!` or Not To `panic!`](./error_handling/panic_or_not_panic//)

</details>

<details>
<summary>Generic Types, Traits, and Lifetimes</summary>

* [Generic Data Types](./generic_types_traits_lifetimes/generic_data_types)
* [Traits](./generic_types_traits_lifetimes/traits)
* [Validating References with Lifetimes](./generic_types_traits_lifetimes/lifetime_syntax)

</details>

<details>
<summary>Testing</summary>

* [Writing tests](./testing/writing_tests)
* [Running tests](./testing/running_tests)
* [Test Organization](./testing/test_organization)

</details>

<details>
<summary>An I/O Project</summary>

* [Minigrep](./an_io_project/minigrep)

</details>

<details>
<summary>Functional Features</summary>

* [Closures](./functional_features/closures)
* [Iterators](./functional_features/iterators)

</details>

<details>
<summary>Cargo</summary>

* [Publishing a Crate](./cargo/publish_a_crate)
* [Demo](./cargo/art)

</details>

<details>
<summary>Smart Pointers</summary>

* [`Box<T>`](./smart_pointers/box_points)
* [`Deref`](./smart_pointers/deref_trait)
* [`Drop`](./smart_pointers/drop_trait)
* [`Rc<T>`](./smart_pointers/rc)
* [`RefCell<T>` and the Interior Mutability Pattern](./smart_pointers/refcell_interior_mutability)
* [Reference Cycles](./smart_pointers/reference_cycles)

</details>

<details>
<summary>Fearless Concurrency</summary>

* [Threads](./fearless_concurrency/threads)
* [Message Passing](./fearless_concurrency/message_passing)
* [Shared State](./fearless_concurrency/shared_state)

</details>

<details>
<summary>OOP</summary>

* [What is OO](./oop/what_is_oo)
* [Trait Objects](./oop/trait_objects)

</details>