#!/bin/sh

echo "Running rustfmt..."
cargo fmt
echo "Running clippy..."
cargo clippy --fix --allow-staged 