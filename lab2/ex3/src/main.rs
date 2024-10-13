fn add_space(s: &mut String, number: i32) {
    let mut index = 0;
    while index < number {
        s.push(' ');
        index += 1;
    }
}

fn add_str(s: &mut String, str: &str) {
    s.push_str(str);
}

fn add_integer(s: &mut String, number: i32) {
    let mut power: i32 = 1;
    let mut index: i32 = 1;
    while number > power {
        power *= 10;
        index += 1;
    }

    if power != number {
        power /= 10;
        index -= 1;
    }

    let letters_number = index;
    while power != 0 {
        if index % 3 == 0 && index != letters_number {
            s.push('_');
        }
        s.push((((number / power) % 10) as u8 + b'0') as char);
        power /= 10;
        index -= 1;
    }
}

fn add_float(s: &mut String, number: f32) {
    let integer: i32 = number as i32;
    let mut fractional: f32 = number - integer as f32;

    let mut power = 1;
    while integer >= power * 10 {
        power *= 10;
    }

    while power > 0 {
        s.push((((integer / power) % 10) as u8 + b'0') as char);
        power /= 10;
    }

    s.push('.');

    let mut count = 0;
    while count < 3 {
        fractional *= 10.0;
        let integer = fractional as i32;
        s.push(((integer % 10) as u8 + b'0') as char);
        fractional -= integer as f32;

        if fractional == 0.0 {
            break;
        }

        count += 1;
    }
}

fn main() {
    let mut result = String::new();

    add_space(&mut result, 40);
    add_str(&mut result, "I 💚\n");
    add_space(&mut result, 40);
    add_str(&mut result, "RUST.\n\n");

    add_space(&mut result, 4);
    add_str(&mut result, "Most");
    add_space(&mut result, 12);
    add_str(&mut result, "crate");
    add_space(&mut result, 6);
    add_integer(&mut result, 306437968);
    add_space(&mut result, 10);
    add_str(&mut result, "and");
    add_space(&mut result, 7);
    add_str(&mut result, "latest");
    add_space(&mut result, 10);
    add_str(&mut result, "is\n");

    add_space(&mut result, 10);
    add_str(&mut result, "downloaded");
    add_space(&mut result, 6);
    add_str(&mut result, "has");
    add_space(&mut result, 14);
    add_str(&mut result, "downloads");
    add_space(&mut result, 5);
    add_str(&mut result, "the");
    add_space(&mut result, 9);
    add_str(&mut result, "version");
    add_space(&mut result, 4);
    add_float(&mut result, 2.038);
    add_str(&mut result, ".\n");

    println!("{}", result);
}
