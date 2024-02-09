# List all available commands
[private]
default:
    @just --list --justfile {{justfile()}}

# run formatter
fmt:
    cargo +nightly fmt --all

# run formatter (check)
fmt-check:
    cargo +nightly fmt --all --check    

# run tests
test: 
    cargo test

# run linters
lint: 
    cargo clippy
