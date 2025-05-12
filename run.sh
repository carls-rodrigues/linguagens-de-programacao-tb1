#!/bin/bash

# This script is used to run the binary_search.rs or main.py file with the specified arguments.
# Usage: ./run.sh [args]
# Example: ./run.sh --size=1000 --target=500

if [ $# -eq 0 ]; then
    echo "No arguments provided. Please provide arguments to run the script."
    exit 1
fi

if [ "$1" = "python" ]; then
    # python3 main.py "${@:2}"
    echo "Compiling Python code..."
    echo "Add your Python compilation command here"
elif [ "$1" = "rust" ]; then
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        echo "Rust is not installed. Please install Rust to run this script."
        echo "Visit https://www.rust-lang.org/tools/install for installation instructions."
        exit 1
    fi
    echo "Compiling Rust code..."
    cargo build --release --manifest-path=rust/Cargo.toml --target-dir=rust/target    
    
    if [ $? -ne 0 ]; then
        echo "Compilation failed. Please check your Rust code."
        exit 1
    fi
    
    ./rust/target/release/rust "${@:2}"
elif [ "$1" = "c" ]; then
    # Check if C compiler is installed
    if ! command -v gcc &> /dev/null; then
        echo "C compiler is not installed. Please install a C compiler to run this script."
        exit 1
    fi
    echo "Compiling C code..."
    gcc -o c/binary_search c/binary_search.c
    
    if [ $? -ne 0 ]; then
        echo "Compilation failed. Please check your C code."
        exit 1
    fi
    
    ./c/binary_search "${@:2}"
else
    echo "Invalid argument. Please specify 'python' or 'rust' as the first argument."
    exit 1
fi