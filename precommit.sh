#!/bin/sh

echo "Running rustfmt..."
cargo fmt
echo "Running clippy..."
cargo clippy --fix --allow-staged -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \