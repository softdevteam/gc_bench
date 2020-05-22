#!/bin/sh
#
# Build script for continuous integration.

set -e

export CARGO_HOME="`pwd`/.cargo"
export RUSTUP_HOME="`pwd`/.rustup"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh --default-host x86_64-unknown-linux-gnu --default-toolchain stable -y --no-modify-path
export PATH=`pwd`/.cargo/bin/:$PATH

# Rustfmt checks
rustup component add rustfmt
cargo fmt --all -- --check

# needed to build benchmarks
cargo install cargo-script

# Ensure the building rustc_boehm fails if it uses excessive amounts of memory.
ulimit -d $((1024 * 1024 * 8)) # 8 GiB

# Build rustc_boehm
git clone https://github.com/softdevteam/rustc_boehm
cd rustc_boehm && ./x.py install --config ../.buildbot.config.toml

# Run the benchmarks. For now, simply running them successfully is enough to
# pass the CI checks.
RUSTC_BOEHM=rustc_boehm/build/rustc_boehm/bin/rustc cargo run --release

