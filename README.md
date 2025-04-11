# {{project-name}}

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/{{handle}}/{{crate_name}}/branch/main/graph/badge.svg)](https://codecov.io/gh/{{handle}}/{{crate_name}})
[![crates.io](https://img.shields.io/crates/v/{{crate_name}})](https://crates.io/crates/{{crate_name}})
[![Docs.rs](https://docs.rs/{{crate_name}}/badge.svg)](https://docs.rs/{{crate_name}})


{{ description }}

## Dev tools

To develop {{project-name}} you'll want to have these tools installed:

- [`just`](https://github.com/casey/just)   [https://github.com/casey/just]  This command runner will let you run our workflows reliably (including installing the other dev dependencies)
- [`typos`](https://github.com/crate-ci/typos)
- [`cargo-semver`](https://github.com/obi1kenobi/cargo-semver-checks)
- [`git-cliff`](https://github.com/orhun/git-cliff)

## Template

This repo was initially setup using [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) and [this template](https://github.com/savente93/rust-template)
