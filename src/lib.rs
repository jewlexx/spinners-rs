#![warn(missing_docs)]
//! # Spinners for Rust
//!
//! [![Cargo version](https://img.shields.io/crates/v/spinners-rs.svg)](https://crates.io/crates/spinners-rs)
//! [![License](https://img.shields.io/crates/l/spinners-rs)](https://github.com/jewlexx/spinners-rs/blob/master/LICENSE)
//! [![Docs](https://img.shields.io/docsrs/spinners-rs)](https://docs.rs/spinners-rs/)
//! [![Downloads](https://img.shields.io/crates/d/spinners-rs.svg)](https://crates.io/crates/spinners-rs)
//!
//! A lightweight collection of 80+ spinners for Rust, designed for speed and minimal overhead.
//!
//! ## Basic Example
//!
//! ```rust
//! use std::{thread, time::Duration};
//! use spinners_rs::{Spinner, Spinners};
//!
//! let mut sp = Spinner::new(Spinners::Arrow, "Doing Some Things...");
//!
//! sp.start();
//!
//! thread::sleep(Duration::from_secs(3));
//! ```

// All of the actual code is stored in other files
// This is the main file that includes the code for the Spinner struct
mod spinner;
// This is the file that includes all the data for the spinners like the spinner enum and the frames
mod spinners;

pub use spinner::*;
pub use spinners::*;
