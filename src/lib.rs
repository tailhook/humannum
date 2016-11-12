extern crate num_traits;
#[macro_use(quick_error)] extern crate quick_error;

mod integer;
mod error;

pub use integer::parse_integer;
pub use error::Error;


pub const NUMERIC_SUFFIXES: &'static [(&'static str, u64)] = &[
    ("k", 1000),
    ("M", 1000_000),
    ("G", 1000_000_000),
    ("ki", 1024),
    ("Mi", 1048576),
    ("Gi", 1024*1024*1024),
    ];
