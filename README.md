# IAMM SDK

This project was generated with [Trampoline](https://github.com/Tempest-Protocol/trampoline).


## Organization

### Generators
High level functions that use a `TContract` to perform operations (by generating transactions) that interact with IAMM NFTs. 

### Schemas
Molecule-defined structures and bindings to rust-native types.

### Scripts
Smart contracts written with [Capsule](https://github.com/nervosnetwork/capsule)

## Build Instructions
To build `generators` and `schemas`, run `cargo build --workspace`.

Ensure you have the necessary pre-requisites installed:

1. install ckb-binary-patcher
    `cargo install --git https://github.com/xxuejie/ckb-binary-patcher.git`
2. add target
    `rustup  target add riscv64imac-unknown-none-elf`
    `apt-get install gcc-riscv64imac-unknown-none-elf`

Then, to build the nft contract: `cd scripts/iamm_nft` and `bash build.sh`.

If the bash script produces errors such as this:
`error: failed to run custom build command for ckb-std v0.9.0`

then you may need to install the the [riscv toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) 

*Note*: We do not use `capsule build` because it prevents using the `iamm_nft-schemas` package as a dependency due to the project structure.