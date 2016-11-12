quick_error! {
    /// Error parsing human-friendly number
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Error {
        Empty {
            description("number is empty")
        }
        InvalidDigit {
            description("invalid digit")
        }
        Overflow {
            description("number is too large or too small (for negative)")
        }
    }
}
