#!/bin/bash
cd .. ..
sudo apt update
sudo apt install build-essential
sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default nightly-2023-01-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-01-01

cargo install --force subkey --git https://github.com/paritytech/substrate --tag=v3.0.0 --locked

printf "For BABE:\n"
subkey generate --scheme SR25519 --words 24 --network substrate
printf "For GRANDPA:\n"
subkey generate --scheme ED25519 --words 24 --network substrate
printf "For IM_ONLINE:\n"
subkey generate --scheme SR25519 --words 24 --network substrate
