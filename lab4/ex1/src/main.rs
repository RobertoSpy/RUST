use std::fs;

fn  longest_line(file_name: &str) -> Result<(), std::io::Error> {

    let text = fs::read_to_string(file_name)?;
    let mut longest_bytes = "";
    let mut longest_characters = "";

    for line in text.lines(){
        if line.as_bytes().len() > longest_bytes.len() {
            longest_bytes = line;
        }

        if line.chars().count() > longest_characters.chars().count() {
            longest_characters = line;
        }
    }
    
    println!("Cea mai lunga secventa pe bytes {}", longest_bytes);
    println!("Cea mai lunga secventa pe caractere {}", longest_characters);

    Ok(())
}

fn main(){
    if let Err(e) = longest_line("src/file.txt") {
        println!("Error: {}", e);
    }
}