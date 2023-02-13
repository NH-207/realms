# Getting Started

Pull down or clone this repository. You can run any examples in any package using `cargo run --example <EXAMPLE_NAME>`.

For example, to run the [basic_controller](./player/examples/basic_controller.rs) example, you can use `cargo run --example basic_controller`.

# Contributing a New Library

Use `cargo new <LIBRARY_NAME> --lib` to create a new library.

Make sure to add this library to the workspaces in the [Cargo.toml](./Cargo.toml)

# Contributing to an Existing Library

At this current stage, everything is unstable and expected to break hard. However, all tests must pass and examples must work as expected.