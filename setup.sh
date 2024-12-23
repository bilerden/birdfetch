#!/bin/bash

# This script will build the Birdfetch binary and run it directly

# CHECK
if ! command -v rustup &> /dev/null
then
    echo "Rust is not installed. Please install Rust using https://www.rust-lang.org/tools/install"
    exit 1
fi

# BUILD
echo "Building the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed. Please check for errors."
    exit 1
fi

#  RUN
echo "Running Birdfetch..."
./target/release/birdfetch
