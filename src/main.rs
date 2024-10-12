mod logger;

use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, OpenOptions, File, read_to_string};
use std::io::{self, stdout, stdin, Read, Write};
use std::path::{Path, PathBuf};

use clipboard::{ClipboardContext, ClipboardProvider};
use clap::Parser;

/// The default path to the main safe storage
const SAFE_CONTENT_PATH: &str = "./.safe/.main.safe";

/// Search for a pattern in the stored passwords
#[derive(Parser,)]
struct Cli {
    /// Which string are you looking for?
    #[arg(short, long, default_value_t=String::from(""))]
    pattern: String,

    /// Command which you want to run
    #[arg(short, long, default_value_t=String::from("search"))]
    command: String,

    /// Key for the password/website
    #[arg(short, long, default_value_t=String::from(""))]
    key: String,

    /// Value of the password, this will get encrypted
    #[arg(long, default_value_t=String::from(""))]
    password: String
}

fn handle_init_safe(file_path: &Path) -> io::Result<(String)> {
    logger::info("Initializing your first safe.");

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(&file_path)?;

    logger::success("Your safe has been initialized successfully!");
    Ok((String::from("Safe initialized")))
}

fn add_line_to_file(file_path: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

fn get_all_safe_keys(file_path: &Path) -> HashMap<String, String> {
    let content = read_to_string(file_path).expect("Unable to read file");
    let mut keys_map = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((key_part, value_part)) = line.split_once('=') {
            let value = value_part.trim_end_matches(';').to_string();
            let key = key_part.trim().to_string();

            keys_map.insert(key, value);
        } else {
            eprintln!("Warning: Invalid line format: {}", line);
        }
    }

    println!("Returning keys map: {:?}", keys_map);
    keys_map
}

fn handle_search(keys_map: HashMap<String, String>, key_pattern: String) {
    println!("Looking for password of pattern: {}", key_pattern.to_string());

    let re = Regex::new(&key_pattern).expect("Invalid regex pattern");
    let matching_keys: Vec<(&String, &String)> = keys_map
        .iter()
        .filter(|(key, _)| re.is_match(key))
        .collect();

    let num_matches = matching_keys.len();

    if num_matches == 0 {
        println!("No matching keys found.");
    } else if num_matches == 1 {
        let (key, password) = matching_keys[0];

        println!("Found one matching key: '{}'", key);
        print!("Do you want to retrieve the password? (y/N): ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_lowercase();
        if input == "y" || input == "yes" || input == "Yes" || input == "Y" {
            println!("Password for '{}': '{}'", key, password);

            if copy_to_clipboard(password).is_ok() {
                println!("Password for '{}' has been copied to clipboard", key);
            }
        } else {
            println!("Password retrieval canceled.");
        }
    } else {
        println!("Multiple matching keys found: ");
        for (key, _) in &matching_keys {
            println!("- {}", key);
        }

        println!("Please provide a more specific pattern to narrow down the search.");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        handle_search(keys_map, input.to_string());
    }
}

fn handle_add_key(file_path: &Path, key: String, password: String) {
    println!("Adding key: {}", key);

    let encrypted_line = format!("{}={};", key, password);
    add_line_to_file(file_path, &encrypted_line);
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Cli::parse();

    if args.command == "search" && args.pattern.is_empty() {
        args.pattern = std::env::args().nth(2).expect("no pattern given");
    }

    println!("command: {:?}, pattern: {:?}", args.command, args.pattern);

    let file_path = Path::new(SAFE_CONTENT_PATH);

    if !file_path.exists() {
        println!("Creating path...");
        handle_init_safe(file_path);
    }

    match String::from(args.command).as_str() {
        "add" => {
            println!("handle_add_key!");
            handle_add_key(file_path, String::from(args.key), String::from(args.password))
        },
        "search" => {
            let keys_map = get_all_safe_keys(file_path);
            handle_search(keys_map, String::from(args.pattern));
        },
        _ => {
            println!("Not matched...");
        }
    }

    Ok(())
}

