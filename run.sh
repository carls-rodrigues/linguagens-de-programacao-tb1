#!/bin/bash

# This script is used to run the binary_search.rs or main.py file with the specified arguments.
# Usage: ./run.sh [args]
# Example: ./run.sh --size 1000 --target 500

if [ $# -eq 0 ]; then
    echo "No arguments provided. Please provide arguments to run the script."
    exit 1
fi

if [ "$1" = "python" ]; then
    # python3 main.py "${@:2}"
    echo "Compiling Python code..."
    echo "Add your Python compilation command here"
elif [ "$1" = "rust" ]; then
    echo "Compiling Rust code..."
    cargo build --release --manifest-path=rust/Cargo.toml --target-dir=rust/target    
    
    if [ $? -ne 0 ]; then
        echo "Compilation failed. Please check your Rust code."
        exit 1
    fi
    
    ./rust/target/release/rust "${@:2}"
else
    echo "Invalid argument. Please specify 'python' or 'rust' as the first argument."
    exit 1
fi