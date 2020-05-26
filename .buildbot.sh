#!/bin/sh
#
# Build script for continuous integration.

set -e

export CARGO_HOME="`pwd`/.cargo"
export RUSTUP_HOME="`pwd`/.rustup"

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh --default-host x86_64-unknown-linux-gnu --default-toolchain nightly -y --no-modify-path

export PATH=`pwd`/.cargo/bin/:$PATH

# Sometimes rustfmt is so broken that there is no way to install it at all.
# Rather than refusing to merge, we just can't rust rustfmt at such a point.
rustup component add --toolchain nightly rustfmt-preview \
    || cargo +nightly install --force rustfmt-nightly \
    || true
rustfmt=0
cargo fmt 2>&1 | grep "not installed for the toolchain" > /dev/null || rustfmt=1
if [ $rustfmt -eq 1 ]; then
    cargo +nightly fmt --all -- --check
fi

# needed to build benchmarks
cargo install cargo-script

# Ensure the building rustc_boehm fails if it uses excessive amounts of memory.
ulimit -d $((1024 * 1024 * 8)) # 8 GiB

# Build rustc_boehm
git clone https://github.com/softdevteam/rustc_boehm
mkdir -p rustc_boehm/build/rustc_boehm
cd rustc_boehm && ./x.py build --stage 1 src/libstd && ./x.py install --config ../.buildbot.config.toml

# Run the benchmarks. For now, simply running them successfully is enough to
# pass the CI checks.
RUSTC_BOEHM=rustc_boehm/build/rustc_boehm/bin/rustc cargo run --release

