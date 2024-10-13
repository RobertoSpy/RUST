fn add_chars_n(sequence: &mut String, character: char, size: i32) {
    let s = sequence;
    let mut index = 0;
    while index <= size {
        s.push(character);
        index += 1;
    }
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
