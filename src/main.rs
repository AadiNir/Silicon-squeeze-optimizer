use std::collections::HashMap;
use std::io;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
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
        fn load_from_file(path: &Path) -> io::Result<Self> {
            if !path.exists(){
                return Ok(SqueezeStore::new());
            }
            let contents = fs::read_to_string(path)?;
            let loaded:SqueezeStore= serde_json::from_str(&contents)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            
            Ok(loaded)

        }

        fn save_to_file(&self, path: &Path) -> io::Result<()> {
            let contents = serde_json::to_string(&self)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            fs::write(path, contents)
        }

}

enum Command{
    Set(String, String),
    Get(String),
    Exit,
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts = input.splitn(3, ' ');

    let command = parts.next().ok_or("Empty command")?;

    match command.to_lowercase().as_str() {
        "set" => {
            let key = parts.next().ok_or("Missing key for 'set'")?;
            let value = parts.next().ok_or("Missing value for 'set'")?;

            Ok(Command::Set(key.into(), value.into()))
        }
        "get" => {
            let key = parts.next().ok_or("Missing key for 'get'")?;

            Ok(Command::Get(key.into()))
        }
        "exit" => Ok(Command::Exit),

        _ => Err("Unknown command".into()),
    }
}

fn main() {
   
    let mut db = SqueezeStore::load_from_file(Path::new("store.json")).expect("Failed to load store from file");
  
    println!("welcome to SqueezeStore! A simple in-memory key-value store.");

    loop{
        println!("Enter a command (set/get/exit):");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        match parse_command(&command){
            Ok(Command::Set(key, value)) => {
                db.set(key, value);
                match db.save_to_file(Path::new("store.json")) {
                    Ok(()) => println!("Key-value pair set successfully."),
                    Err(e) => eprintln!("Error saving to file: {}", e),
                }
            },
            Ok(Command::Get(key)) => {
                match db.get(&key){
                    Some(val) => println!("Value for '{}': {}", key, val),
                    None => println!("Key '{}' not found", key),
                }
            },
            Ok(Command::Exit) => {
                match db.save_to_file(Path::new("store.json")) {
                    Ok(()) => println!("Exiting..."),
                    Err(e) => eprintln!("Error saving to file: {}", e),
                }
                break;
            },
            Err(e) => println!("Error: {}", e),
        }


    }
}
