#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

rustup target add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl
