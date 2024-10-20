fn checked_addition(number1: u32, number2: u32) -> u32 {
    if (number1 as u64) + (number2 as u64) <= u32::MAX as u64 {
        return number1 + number2;
    } else {
        panic!("S-a panicat suma!");
    }
}

fn checked_multiplication(number1: u32, number2: u32) -> u32 {
    if (number1 as u64) * (number2 as u64) <= u32::MAX as u64 {
        return number1 * number2;
    } else {
        panic!("S-a panicat multiplicarea!");
    }
}

fn main() {
    let final_sum = checked_addition(394230900, 999953435);
    println!("suma {}", final_sum);

    let final_mult = checked_multiplication(1222222, 104);
    println!("multiplicarea {}", final_mult);
}
