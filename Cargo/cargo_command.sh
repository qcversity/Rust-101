#!/bin/bash

# Common Cargo Commands Bash Script

# Check the cargo help page
cargo --help 

# Create a new Rust project
cargo new my_project

# Change directory to the project folder
cd my_project

# Check the project for common issues
cargo check

# Test the project
cargo test


# Build the project (debug mode)
cargo build

# Create a release build
cargo build --release

# Format the source code using Rustfmt
cargo fmt

# Run the project
cargo run

# List available dependencies and their versions
cargo tree


# Check for and apply updates to project dependencies
cargo update

# Clean the project, removing build artifacts
cargo clean


# Generate project documentation
cargo doc


# Publish the project to crates.io
# (Note: Requires you to be the owner of the package on crates.io)
# cargo publish

# Navigate back to the parent directory
cd ..

# Remove the project directory
# Note: Use with caution, as this will delete the 'my_project' directory
# rm -r my_project

echo "Common Cargo commands have been demonstrated in this script."

# You can make this files executable using `chmod +x` test_project.sh, and then run it with ./test_project.sh in your terminal.