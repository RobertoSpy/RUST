fn numbers_div(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
fn numbers_is_coprime(a: i32, b: i32) -> bool {
    numbers_div(a, b) == 1
}

fn main() {
    let mut indexi = 0;
    let mut indexj;
    let mut vector = Vec::new();
    let mut numbers = 1;
    while numbers <= 100 {
        vector.push(numbers);
        numbers += 1;
    }

    while indexi < vector.len() - 1 {
        indexj = indexi + 1;
        while indexj < vector.len() {
            if numbers_is_coprime(vector[indexi], vector[indexj]) {
                println!("{} cu {}", vector[indexi], vector[indexj]);
            }
            indexj += 1;
        }
        indexi += 1;
    }
}
