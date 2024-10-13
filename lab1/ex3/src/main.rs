fn main() {
    let mut number = 99;
    while number > 1 {
        println!(
            "{} bottles of beer on the wall,\n{} bottles of beer.",
            number, number
        );
        println!(
            "Take one down, pass it around,\n{} bottles of beer on the wall.\n",
            number - 1
        );
        number = number - 1;
    }
    println!(
        "{} bottle of beer on the wall,\n{} bottle of beer.",
        number, number
    );
    println!("Take one down, pass it around,\nNo bottles of beer on the wall.");
}
