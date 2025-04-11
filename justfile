#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

log := "warn"

alias b := build
alias t := test
alias c := check
alias l := lint

export JUST_LOG := log

lint:
    cargo clippy --all --all-targets --all-features -- --deny warnings
    cargo fmt --all -- --check
    typos -w .
    taplo fmt . 
    

# Run tests
test:
    cargo test --all

# Build the project
build:
    cargo build

# Build the project
build-release:
    cargo build --release 

doc:
    cargo doc --no-deps --all-features --workspace

# Clean the target directory
clean:
    cargo clean

# Check for errors without building (quick dev check)
check:
    cargo check

update:
    cargo update

newest:
    cargo +nightly update --breaking -Z unstable-options

semver:
    cargo semver-checks

install-dev-tools:
    cargo install cargo-binstall 
    cargo binstall cargo-semver -y
    cargo binstall git-cliff -y
    cargo binstall typos-cli -y
    cargo binstall taplo-cli -y
    cargo binstall bacon -y

# Publish the crate to crates.io
# Add confirmation to prevent accidental publishing
publish:
    echo "Are you sure you want to publish? (y/n)"
    read -r confirm
    if [[ $confirm == "y" || $confirm == "Y" ]]; then
        cargo publish
    else
        echo "Publish cancelled"
    fi

# Run all quality checks: fmt, lint, check, test
ci:
    just lint
    just check
    just test

