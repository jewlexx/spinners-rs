# Spinners for Rust

[![Cargo version](https://img.shields.io/crates/v/spinners-rs.svg)](https://crates.io/crates/spinners-rs)
[![License](https://img.shields.io/github/license/jewlexx/spinners-rs)](https://github.com/jewlexx/spinners-rs/blob/master/LICENSE)
[![Docs](https://img.shields.io/badge/docs-👌-4EC329.svg?)](https://docs.rs/spinners-rs/)
[![Downloads](https://img.shields.io/crates/d/spinners-rs.svg)](https://crates.io/crates/spinners-rs)

![Demo Gif](./demo/render.gif)

## Install

See [Cargo page](https://crates.io/crates/spinners-rs)

## Usage

```rust
use std::{thread, time::Duration};
use spinners_rs::{Spinner, Spinners};

let mut sp = Spinner::new(Spinners::Arrow, "Doing Some Things...").unwrap();

sp.start();

thread::sleep(Duration::from_secs(3));
```

- [List of available spinners](src/spinners.rs)
- [Documentation](https://docs.rs/spinners-rs/)

## Example

```shell
cargo run --example cycle
```

```shell
cargo run --example spin
```

**Made with 💗 by Juliette Cordor**