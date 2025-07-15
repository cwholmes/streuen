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

## Run Dev

```sh
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve
```
