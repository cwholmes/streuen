# Decentral Text

A simple webapp text chat application built to be deployed on IPFS as
a decentralized application.

## Setup

- [trunk](https://trunkrs.dev/#install)

Note: If using `asdf` for the rust install this command will
automatically add the installed `trunk` command to your path as
a shim.

```sh
asdf reshim rust
```

## Run Build

Run to enable to wasm target.

```sh
rustup target add wasm32-unknown-unknown
```

## Build

```sh
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown --bin streuen-chat-web
```

## Run Dev

```sh
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve
```
