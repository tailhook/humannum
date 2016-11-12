use std::str::FromStr;
use std::ops::Deref;

use num_traits::int::PrimInt;

use integer::parse_integer;
use error::Error;

/// A wrapper for integers that has `FromStr` implementation
///
/// This is useful if you want to use it somewhere where `FromStr` is
/// expected.
///
/// See `parse_integer` for the description of the format.
///
/// # Example
///
/// ```
/// use humannum::Int;
///
/// let x: Int<u64> = "  12_500 M  ".parse().unwrap();
/// assert_eq!(*x, 12_500_000_000)
/// ```
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Int<T: PrimInt>(T);

impl<T: PrimInt> AsRef<T> for Int<T> {
    fn as_ref(&self) -> &T { &self.0 }
}

impl<T: PrimInt> Deref for Int<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T: PrimInt> From<T> for Int<T> {
    fn from(val: T) -> Int<T> { Int(val) }
}

impl<T: PrimInt> FromStr for Int<T> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Int<T>, Error> {
        parse_integer(s).map(Int)
    }
}
