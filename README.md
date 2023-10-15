# Rinha de Frontend

Client side in Rust with WebAssembly

## Requirements 📦
1. [Rust 🦀](https://www.rust-lang.org/tools/install)
2. [Wasm](https://rustwasm.github.io/wasm-pack/installer)
3. [Trunk](https://trunkrs.dev)

## Setup 🚀

- 1 Install Wasm
```sh
rustup target add wasm32-unknown-unknown
```
- 2 Install Trunk
```sh
cargo install --locked trunk
```
- 3 Run Trunk
```sh
make run
```

## Build 🏗️
```sh
make build
```