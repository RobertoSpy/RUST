use std::fs::File;
use std::io::{self, BufRead};

trait Command {
    fn get_name(&self) -> String;
    fn exec(&mut self, args: &[String]);
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> String {
        "ping".to_string()
    }

    fn exec(&mut self, args: &[String]) {
        if !args.is_empty() {
            eprintln!("Ping nu are parametrii.");
            return;
        }
        println!("pong!");
    }
}

struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> String {
        "count".to_string()
    }

    fn exec(&mut self, args: &[String]) {
        println!("count {} args", args.len());
    }
}

struct TimesCommand {
    count: u32
}

impl TimesCommand {
    fn new() -> Self {
        TimesCommand { count: 0 }
    }
}

impl Command for TimesCommand {
    fn get_name(&self) -> String {
        "times".to_string()
    }

    fn exec(&mut self, args: &[String]) {
        if !args.is_empty() {
            eprintln!("Times nu are parametrii.");
            return;
        }
        self.count += 1;
        println!("called {} times", self.count);
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
    stop_flag: bool,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
            stop_flag: false,
        }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self, file_path: &str) {
        if let Ok(file) = File::open(file_path) {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => {
                        eprintln!("Eroare citire linie.");
                        continue;
                    }
                };
                let mut parts = Vec::new();
                let words = line.split_whitespace();
                for word in words {
                    parts.push(word.to_string());
                }

                if parts.is_empty() {
                    continue;
                }

                let command_name = &parts[0];
                let args = &parts[1..];

                if command_name == "stop" {
                    self.stop_flag = true;
                    break;
                }

                if let Some(command) = self.find_command(command_name) {
                    command.exec(args);
                } else {
                    eprintln!("Unknown comanda: '{}'.", command_name);
                    if let Some(suggestion) = self.suggest_command(command_name) {
                        println!("Ai vrut asta: '{}'? ", suggestion);
                    }
                }
            }

            if self.stop_flag {
                println!("The End.");
            }
        } else {
            eprintln!("Eroare deschidere fișier.");
        }
    }

    fn find_command(&mut self, input: &str) -> Option<&mut Box<dyn Command>> {
        let input_lower = input.to_lowercase();
        for command in &mut self.commands {
            if command.get_name().to_lowercase() == input_lower {
                return Some(command);
            }
        }
        None
    }

    fn suggest_command(&self, input: &str) -> Option<String> {
        let input_lower = input.to_lowercase();

        for command in &self.commands {
            let command_name_lower = command.get_name().to_lowercase();
            if input_lower == command_name_lower && input != command.get_name() {
                return Some(command.get_name());
            }
        }

        None
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand::new()));

    let file_path = "file.txt";
    terminal.run(file_path);
}
