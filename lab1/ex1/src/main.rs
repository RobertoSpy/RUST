fn number_is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let mut index = 0;

    while index <= 100 {
        if number_is_prime(index) {
            println!("{}", index);
        }
        index += 1;
    }
}
