use num_traits::int::PrimInt;

use {Error, NUMERIC_SUFFIXES};


/// Parse integer value from string
pub fn parse_integer<I: PrimInt>(mut src: &str) -> Result<I, Error> {
    let mut mult = I::from(1).unwrap();
    for &(suffix, value) in NUMERIC_SUFFIXES.iter() {
        if suffix.len() < src.len() &&
            &src[(src.len() - suffix.len())..] == suffix
        {
            mult = I::from(value).ok_or(Error::Overflow)?;
            src = &src[..(src.len() - suffix.len())];
            break;
        }
    }
    let val: I = if src.len() > 2 {
        match &src[..2] {
            "0x" => from_str_radix(&src[2..], 16)?,
            "0o" => from_str_radix(&src[2..], 8)?,
            "0b" => from_str_radix(&src[2..], 2)?,
            _ => from_str_radix(src, 10)?
        }
    } else {
        from_str_radix(src, 10)?
    };
    return val.checked_mul(&mult).ok_or(Error::Overflow);
}

fn from_str_radix<T: PrimInt>(src: &str, radix: u32) -> Result<T, Error> {
    assert!(radix >= 2 && radix <= 36,
           "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
           radix);
    let tradix = T::from(radix).unwrap();

    if src.is_empty() {
        return Err(Error::Empty)
    }

    let is_signed_ty = T::zero() > T::min_value();

    // all valid digits are ascii, so we will just iterate over the utf8 bytes
    // and cast them to chars. .to_digit() will safely return None for anything
    // other than a valid ascii digit for the given radix, including the first-byte
    // of multi-byte sequences
    let src = src.as_bytes();

    let (is_positive, digits) = match src[0] {
        b'+' => (true, &src[1..]),
        b'-' if is_signed_ty => (false, &src[1..]),
        _ => (true, src),
    };

    if digits.is_empty() {
        return Err(Error::Empty)
    }

    let mut result = T::zero();
    if is_positive {
        // The number is positive
        for &c in digits {
            let x = (c as char).to_digit(radix).ok_or(Error::InvalidDigit)?;
            let x = T::from(x).ok_or(Error::Overflow)?;
            result = result.checked_mul(&tradix).ok_or(Error::Overflow)?;
            result = result.checked_add(&x).ok_or(Error::Overflow)?;
        }
    } else {
        // The number is negative
        for &c in digits {
            let x = (c as char).to_digit(radix).ok_or(Error::InvalidDigit)?;
            let x = T::from(x).ok_or(Error::Overflow)?;
            result = result.checked_mul(&tradix).ok_or(Error::Overflow)?;
            result = result.checked_sub(&x).ok_or(Error::Overflow)?;
        }
    }
    Ok(result)
}


#[cfg(test)]
mod test {
    use {parse_integer};

    #[test]
    fn units() {
        assert_eq!(parse_integer::<u64>("12k").unwrap(), 12000);
        assert_eq!(parse_integer::<u64>("157M").unwrap(), 157_000_000);
        assert_eq!(parse_integer::<u64>("2237G").unwrap(), 2_237_000_000_000);
        assert_eq!(parse_integer::<u64>("12ki").unwrap(), 12 << 10);
        assert_eq!(parse_integer::<u64>("157Mi").unwrap(), 157 << 20);
        assert_eq!(parse_integer::<u64>("2237Gi").unwrap(), 2_237 << 30);
    }
}
