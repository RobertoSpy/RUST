#[derive(Debug)]
enum MyError {
    NotAscii,
    NotDigit,
    NotDigit16,
    NotLetter,
    NotPrintable,
}
fn to_uppercase(character: char) -> Result<char, MyError> {
    if character >= 'a' && character <= 'z' {
        return Ok((character as u8 - 32 as u8) as char);
    }
    if character >= 'A' && character <= 'Z' {
        return Ok(character);
    }
    Err(MyError::NotLetter)
}
fn to_lowercase(character: char) -> Result<char, MyError> {
    if character >= 'A' && character <= 'Z' {
        return Ok((character as u8 + 32 as u8) as char);
    }

    if character >= 'a' && character <= 'z' {
        return Ok(character);
    }
    Err(MyError::NotLetter)
}
fn print_char(character: char) -> Result<char, MyError> {
    if character >= '!' && character <= '~' {
        return Ok(character);
    }
    Err(MyError::NotPrintable)
}

fn char_to_number(character: char) -> Result<i32, MyError> {
    if character as u8 <= 0 as u8 || character as u8 >= 127 as u8 {
        return Err(MyError::NotAscii);
    }
    if character as u8 >= 48 as u8 && character as u8 <= 57 as u8 {
        return Ok((character as u8 - b'0') as i32);
    }
    Err(MyError::NotDigit)
}

fn char_to_number_hex(character: char) -> Result<i32, MyError> {
    if character as u8 <= 0 as u8 || character as u8 >= 127 as u8 {
        return Err(MyError::NotAscii);
    }
    if character as u8 >= 0 as u8 && character as u8 <= 9 as u8 {
        return Ok((character as u8 - b'0') as i32);
    }
    if character >= 'A' && character <= 'F' {
        return Ok((character as u8 - b'A' + 10 as u8) as i32);
    }
    if character >= 'a' && character <= 'f' {
        return Ok((character as u8 - b'a' + 10 as u8) as i32);
    }
    Err(MyError::NotDigit16)
}
fn print_error() {
    let _1 = to_uppercase('a');
    let _2 = to_lowercase('/');
    let _3 = print_char('a');
    let _4 = char_to_number('4');
    let _5 = char_to_number_hex('X');
    println!("uppercase {:?}", _1);
    println!("lowercase {:?}", _2);
    println!("print_char {:?}", _3);
    println!("char_to_number {:?}", _4);
    println!("char_to_numberhexa {:?}", _5);
}

fn main() {
    print_error();
}
