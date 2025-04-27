# rvmx

Instrumentatino hypervisor

## setup

First, make sure `rustc` and `cargo` are installed.

To ensure local compliance with repository level policies, it's recommended to set up local `git` hooks:

```
cargo run -p xtask -- install-hooks # populate git hooks (commit, commit message, push)
cargo run -p xtask -- install-tools # locally install dev dependencies
cargo run -p xtask -- build-rvmx # Build the actual rvmx binary blob
```

## development utilities

As this repo is a template for real projects, development environment examples are included (currently VS Code only).

Plugins in use that apply are the following:

- `rust-analyzer` version 0.4.2438
- `black formatter` version 2025.2.0
- `CodeLLDB` version 1.11.4

Rust source for this project was created with the following toolchain versions in use:

```
rustc --version --verbose
rustc 1.88.0-nightly (10fa3c449 2025-04-26)
binary: rustc
commit-hash: 10fa3c449f6b1613b352a6cbf78d3d91fd9a1d81
commit-date: 2025-04-26
host: x86_64-pc-windows-msvc
release: 1.88.0-nightly
LLVM version: 20.1.2
```
