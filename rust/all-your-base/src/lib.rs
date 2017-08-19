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
#[allow(unused_variables)]
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, &'static str> {

    if from_base <= 1 || to_base <= 1 {
        return Result::Err("Base cannot be 0 or 1");
    }

    if number.iter().any(|&d| d >= from_base ) {
        return Err("Digit can not be greater than base");
    }

    let num_digits_src = number.len() as u32;
    let mut base10_num = number.iter().fold((0, 0u32), |(mut base10, index), n| {
        base10 += number[index as usize] * from_base.pow(num_digits_src - index - 1);
        (base10, index + 1)
    }).0;

    if base10_num == 0 {
        return Ok(Vec::new());
    }

    for n in number {
        println!("{}", n);
    }
    println!("base10: {}", base10_num);
    let mut base_dst_num: Vec<u32> = Vec::new();
    let num_digits_dst = (base10_num as f64).log(to_base as f64) as u32 + 1;
    let mut index = num_digits_dst;
    while index > 0 {
        index -= 1;
        let divisor = to_base.pow(index);
        base_dst_num.push(base10_num/divisor);
        base10_num %= divisor;
    }

    Ok(base_dst_num)
}
