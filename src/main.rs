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
fn parse_command(input: &str) -> Option<Command>{
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    if parts.is_empty() {
        return None;
    }

    match parts[0].to_lowercase().as_str() {
        "set" if parts.len() == 3 => Some(Command::Set(parts[1].to_string(), parts[2].to_string())),
        "get" if parts.len() == 2 => Some(Command::Get(parts[1].to_string())),
        "exit" if parts.len() == 1 => Some(Command::Exit),
        _ => None,  
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
            Some(Command::Set(key, value)) => {
                db.set(key, value);
                println!("Key-value pair set successfully.");
            },
            Some(Command::Get(key)) => {
                match db.get(&key){
                    Some(val) => println!("Value for '{}': {}", key, val),
                    None => println!("Key '{}' not found", key),
                }
            },
            Some(Command::Exit) => {
                println!("Exiting...");
                break;
            },
            None => println!("Invalid command. Please try again."),
        }


    }
}
