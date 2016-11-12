quick_error! {
    /// Error parsing human-friendly number
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Error {
        /// Number is completely empty except whitespace and minus sign
        Empty {
            description("number is empty")
        }
        /// One of the characters in the string is not digit
        InvalidDigit {
            description("invalid digit")
        }
        /// A number is too large or too small to fit the type
        Overflow {
            description("number is too large or too small (for negative)")
        }
    }
}
