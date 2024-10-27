use std::fs;

fn replace(input: &str) -> String {
    let abbreviation = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let word: Vec<&str> = input.split_whitespace().collect();
    let mut final1 = Vec::new();

    for words in word {
        let mut contor = false;
        for index in abbreviation.iter(){
            let(mini, full) = *index;
            if words == mini {
                final1.push(full);
                contor = true;
                break;
            }
        }

        if contor == false {
            final1.push(words);
        }
    }

    final1.join(" ")
}

fn main(){
    let input = fs::read_to_string("src/file.txt").expect("Unable to read file");
    let output = replace(&input);
    println!("Varianta corectata\n{}", output);
}