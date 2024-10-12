mod logger;

use std::fs::{self, OpenOptions, File, read_to_string};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

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

fn get_all_safe_keys(file_path: &Path) {
    let content = read_to_string(file_path).expect("Unable to read file");
    println!("{}", content);
}

fn handle_search(file_path: &Path, key_pattern: String) {
    println!("Looking for password of pattern: {}", key_pattern.to_string());
    get_all_safe_keys(file_path);
}

fn handle_add_key(file_path: &Path, key: String, password: String) {
    println!("Adding key: {}", key);

    let encrypted_line = format!("{}={};", key, password);
    add_line_to_file(file_path, &encrypted_line);
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
            handle_search(file_path, String::from(args.pattern));
        },
        _ => {
            println!("Not matched...");
        }
    }

    Ok(())
}

