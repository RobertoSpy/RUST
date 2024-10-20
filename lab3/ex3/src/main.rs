#[derive(Debug)]
enum MyError {
    Overflow,
}
fn checked_addition(number1: u32, number2: u32) -> Result<u32, MyError> {
    if (number1 as u64) + (number2 as u64) <= u32::MAX as u64 {
        Ok(number1 + number2)
    } else {
        Err(MyError::Overflow)
    }
}

fn checked_multiplication(number1: u32, number2: u32) -> Result<u32, MyError> {
    if (number1 as u64) * (number2 as u64) <= u32::MAX as u64 {
        Ok(number1 * number2)
    } else {
        Err(MyError::Overflow)
    }
}
fn propagates_errors() {
    let result1 = checked_addition(3999999999, 999999999);
    let result2 = checked_multiplication(999999, 1);
    println!("Suma: {:?}\nMultiplicarea: {:?}", result1, result2);
}

fn main() {
    propagates_errors();
}
