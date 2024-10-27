fn rot_13 (input: &str) -> Result<String, &str> {
    let mut final_rez = String::new();

    for ch in input.chars() {
        if !ch.is_ascii() {
             return Err(" Nu Ascii");
        }

    let pre_final = match ch {
        'A'..='Z' => (((ch as u8 -b'A' + 13 as u8) % 26) + b'A') as char,
        'a'..='z' => (((ch as u8 - b'a'+ 13 as u8) % 26) + b'a')as char,
        _ => ch,
    };
    
    final_rez.push(pre_final);

}

    Ok(final_rez)
 }


 fn main(){

    let try1 = rot_13("ANA are MeRe");
    let try2 = rot_13("🎁🎶🎉👀🎈🎃🍕☕🍉");
    println!("{:?}", try1);
    println!("{:?}", try2);
 }
