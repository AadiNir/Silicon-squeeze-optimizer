use std::collections::HashMap;
use std::io;

struct SqueezeStore{
        store: HashMap<String, String>,
}
impl SqueezeStore {
        fn new() -> Self {
            SqueezeStore {
                store: HashMap::new(),
            }
        }

        fn get(&self, key:&str)-> Option<&str> {
            self.store.get(key).map(|s| s.as_str())
        }
        fn set(&mut self, key: String, value: String) {
            self.store.insert(key, value);
        }
}

enum Command{
    Set(String, String),
    Get(String),
    Exit,
}
fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty command".into());
    }

    match parts[0].to_lowercase().as_str() {
        "set" => {
            if parts.len() == 3 {
                Ok(Command::Set(parts[1].into(), parts[2].into()))
            } else {
                Err("Invalid 'set' command. Use: set <key> <value>".into())
            }
        }
        "get" => {
            if parts.len() == 2 {
                Ok(Command::Get(parts[1].into()))
            } else {
                Err("Invalid 'get' command. Use: get <key>".into())
            }
        }
        "exit" => {
            if parts.len() == 1 {
                Ok(Command::Exit)
            } else {
                Err("Invalid 'exit' command. Use: exit".into())
            }
        }
        _ => Err("Unknown command".into()),
    }
}
fn main() {
   
    let mut db = SqueezeStore::new();
    db.set("name".to_string(), "Aadithya".to_string());
    match db.get("name"){
        Some(val)=> println!("Value for 'name': {}", val),
        None => println!("Key 'name' not found"),
    }

    println!("welcome to SqueezeStore! A simple in-memory key-value store.");


    loop{
        println!("Enter a command (set/get/exit):");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        match parse_command(&command){
            Ok(Command::Set(key, value)) => {
                db.set(key, value);
                println!("Key-value pair set successfully.");
            },
            Ok(Command::Get(key)) => {
                match db.get(&key){
                    Some(val) => println!("Value for '{}': {}", key, val),
                    None => println!("Key '{}' not found", key),
                }
            },
            Ok(Command::Exit) => {
                println!("Exiting...");
                break;
            },
            Err(e) => println!("Error: {}", e),
        }


    }
}
