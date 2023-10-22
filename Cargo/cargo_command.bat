REM Common Cargo Commands Batch Script
echo "==============================="

REM Check the cargo help page
cargo --help 

REM Create a new Rust project
cargo new my_project

REM Change directory to the project folder
cd my_project

REM Check the project for common issues
cargo check

REM Test the project
cargo test

REM Build the project (debug mode)
cargo build

REM Create a release build
cargo build --release

REM Format the source code using Rustfmt
cargo fmt

REM Run the project
cargo run

REM List available dependencies and their versions
cargo tree

REM Check for and apply updates to project dependencies
cargo update

REM Clean the project, removing build artifacts
cargo clean

REM Generate project documentation
cargo doc

REM Publish the project to crates.io
REM (Note: Requires you to be the owner of the package on crates.io)
REM cargo publish

REM Navigate back to the parent directory
cd ..

REM Remove the project directory
REM Note: Use with caution, as this will delete the 'my_project' directory
rmdir /s /q my_project

echo "Common Cargo commands have been demonstrated in this script."
