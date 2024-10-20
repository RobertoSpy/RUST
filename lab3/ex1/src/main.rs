fn is_prime(number: u16) -> bool {
    let mut index = 2;
    if number < 2 {
        return false;
    }
    while (index as u32) * (index as u32) <= (number as u32) {
        if number % index == 0 {
            return false;
        }
        index += 1;
    }
    true
}

fn next_prime(number: u16) -> Option<u16> {
    let mut successor = number + 1;
    while successor < 65535 {
        if is_prime(successor) == true {
            return Some(successor);
        }
        successor += 1;
    }
    None
}

fn main() {
    let mut final_number = 0u16;
    while let Some(i) = next_prime(final_number) {
        println!("{}", i);
        final_number = i;
    }
    println!("S-a terminat!");
}
