use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "C:\\Windows\\System32\\drivers\\etc\\hosts"; 
    let content = fs::read_to_string(path)?;

    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
         
            println!("{} => {}", parts[1], parts[0]);
        }
    }

    Ok(())
}
