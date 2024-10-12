mod logger;

use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, OpenOptions, File, read_to_string};
use std::io::{self, stdout, stdin, Write};
use std::path::{Path};

use clipboard::{ClipboardContext, ClipboardProvider};

/// The default path to the main safe storage.
const SAFE_CONTENT_PATH: &str = "./.safe/.main.safe";

/// Initializes the .safe directory (creates the dir and the .main.safe file.)
fn handle_init_safe(file_path: &Path) -> io::Result<String> {
    logger::info("Initializing your first safe.");

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let _file = File::create(&file_path)?;

    logger::success("Your safe has been initialized successfully!");
    Ok(String::from("Safe initialized"))
}

/// A utility method which opens the file in append mode and adds a new line with "content".
fn add_line_to_file(file_path: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

/// A method which reads the .main.safe file and returns all the key/value pairs detected
/// (encrypted.)
fn get_all_safe_keys(file_path: &Path) -> HashMap<String, String> {
    let content = read_to_string(file_path).expect("Unable to read file");
    let mut keys_map = HashMap::new();
    let mut line_number = 0;

    for line in content.lines() {
        line_number += 1;

        if line.trim().is_empty() {
            continue;
        }

        if let Some((key_part, value_part)) = line.split_once('=') {
            let value = value_part.trim_end_matches(';').to_string();
            let key = key_part.trim().to_string();

            keys_map.insert(key, value);
        } else {
            logger::warn(&format!("Warning: invalid line format at line number: {}. Please investigate or contact support.", line_number));
        }
    }

    logger::debug(&format!("Returning keys map: {:?}", keys_map));
    keys_map
}

fn handle_single_match(key: &str, password: &str) {
    logger::infosn(&format!("Found one key '{}'. Do you want to retrieve the password? (y/N): ", key));
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim().to_lowercase();
    if input == "y" || input == "yes" || input == "Yes" || input == "Y" {
        if copy_to_clipboard(password).is_ok() {
            logger::success(&format!("Password for '{}' has been copied to clipboard.", key));
        }
    } else {
        logger::info("Password retrieval canceled.");
        std::process::exit(1);
    }
}

/// This method handles the "search" command; we display all the found keys matching the user
/// phrase and we prompt the user to choose until there is only one left.
fn handle_search(keys_map: HashMap<String, String>, key_pattern: String) {
    let re = Regex::new(&key_pattern).expect("Invalid regex pattern");
    let matching_keys: Vec<(&String, &String)> = keys_map
        .iter()
        .filter(|(key, _)| re.is_match(key))
        .collect();

    let num_matches = matching_keys.len();

    if num_matches == 0 {
        logger::warn("No matching keys found. Here's a list of all the available keys that you saved:");

        for (key, _) in &keys_map {
            logger::info(&format!("\t- {}", key));
        }

        logger::infosn("Please provide a proper key to look for the password: ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        handle_search(keys_map, input.to_string());

        std::process::exit(1);
    } else if num_matches == 1 {
        let (key, password) = matching_keys[0];
        handle_single_match(key, password);
    } else {
        // Let's find out whether there is an exact match
        // if yes, handle single match
        // if not, proceed with the multiple matching approach
        for (key, pswd) in &matching_keys {
            if **key == key_pattern {
                handle_single_match(key, pswd);
            }
        }

        logger::info("Multiple matching keys found: ");
        for (key, _) in &matching_keys {
            logger::info(&format!("\t- {}", key));
        }

        logger::infosn("Please provide a more specific pattern to narrow down the search: ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        handle_search(keys_map, input.to_string());
    }
}

fn handle_add_key(file_path: &Path, key: String, password: String) {
    let encrypted_line = format!("{}={};", key, password);
    add_line_to_file(file_path, &encrypted_line);
    logger::success(&format!("Successfully added encrypted password for key '{}' to your safe. You can now retrieve its value.", key));
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(SAFE_CONTENT_PATH);

    if !file_path.exists() {
        handle_init_safe(file_path);
    }

    let first_arg = std::env::args().nth(1).unwrap_or_else(|| {
        String::from("search")
    });

    let second_arg = std::env::args().nth(2).unwrap_or_else(|| {
        String::from("")
    });

    let third_arg = std::env::args().nth(3).unwrap_or_else(|| {
        String::from("")
    });

    if first_arg == "search" {
        let keys_map = get_all_safe_keys(file_path);
        handle_search(keys_map, String::from(second_arg));
    } else if first_arg == "add" {
        if second_arg.is_empty() {
            logger::error("Key cannot be empty. Please give a correct 'safe add <key> <password>'");
            std::process::exit(1);
        }

        if third_arg.is_empty() {
            logger::error("Password cannot be empty. Please give a correct 'safe add <key> <password>' command.");
            std::process::exit(1);
        }

        handle_add_key(file_path, String::from(second_arg), String::from(third_arg))
    }
    // if no command provided, we assume that it's search and
    // the second argument is the phrase we're looking for
    else if first_arg != "search" && first_arg != "add" {
        let keys_map = get_all_safe_keys(file_path);
        handle_search(keys_map, String::from(first_arg));
    }

    Ok(())
}
