
#[derive(Debug, PartialEq, Eq)]
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
/// Return the corresponding Error enum if the conversion is impossible.
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
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <=1 {
        return Err(Error::InvalidOutputBase);
    }
    
    for &num in number {
        if num >= from_base {
            return Err(Error::InvalidDigit(num));
        }
    }

    let mut digit_position = 0;
    let digit_in_decimal: u32 = number.iter().rev().map(|digit| {
        digit_position += 1;
        return digit * from_base.pow(digit_position - 1)}).sum();
    
    // let mut number_of_digit = 0;
    // for i in 1.. {
    //     if digit_in_decimal / to_base.pow(i) <= 0 {
    //         number_of_digit = i + 1;
    //         break;
    //     }
    // }

    if digit_in_decimal == 0 {
        return Ok(vec![0]);
    }

    let number_of_digit = digit_in_decimal.ilog(to_base) + 1;

    //let result = Vec::new();
    Ok(vec![digit_in_decimal, number_of_digit]) // Placeholder, replace with actual conversion logic
}