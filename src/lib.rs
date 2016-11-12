//! Human-friendly number parser
//!
//! Currently this only currently implements parsing of integers.
//!
//! The format of values accepted is described in docstring
//! of `parse_integer`.
//!
//! # Example (Functional)
//!
//! ```
//! use humannum::parse_integer;
//! let x: u64 = parse_integer("1_320 k").unwrap();
//! assert_eq!(x, 1320000);
//! ```
//!
//! # Example (wrapper)
//!
//! ```
//! use humannum::Int;
//! let x: Int<u32> = "0o1777".parse().unwrap();
//! assert_eq!(*x, 0o1777)
//! ```

#![warn(missing_docs)]

extern crate num_traits;
#[macro_use(quick_error)] extern crate quick_error;

mod integer;
mod error;
mod wrapint;

pub use integer::parse_integer;
pub use error::Error;
pub use wrapint::Int;

/// Numeric suffixes supported by the library
pub const NUMERIC_SUFFIXES: &'static [(&'static str, u64)] = &[
    ("k", 1000),
    ("M", 1000_000),
    ("G", 1000_000_000),
    ("ki", 1024),
    ("Mi", 1048576),
    ("Gi", 1024*1024*1024),
    ];
