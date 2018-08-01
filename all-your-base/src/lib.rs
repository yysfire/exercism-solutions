#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);

    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    for &n in number {
        if n >= from_base {
            return Err(Error::InvalidDigit(n));
        }
    }

    let mut num = number.iter()
        .skip_while(|x| **x == 0)
        .fold(0, |acc, x| acc * from_base + x);

    let mut result = vec![];
    while num != 0 {
        let remainder = num % to_base;
        result.push(remainder);
        num /= to_base;
    }

    Ok(result.into_iter().rev().collect())
}
