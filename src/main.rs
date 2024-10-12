mod logger;

use std::fs::{self, OpenOptions, File};
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
}

fn handle_init_safe(file_path: &Path) -> io::Result<(String)> {
    logger::info("Initializing your first safe.");

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&file_path)?;
    file.write_all(b"Initial content")?;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Cli::parse();

    if args.command == "search" && args.pattern.is_empty() {
        args.pattern = std::env::args().nth(2).expect("no pattern given");
    }

    println!("command: {:?}, pattern: {:?}", args.command, args.pattern);


    let file_path = Path::new(SAFE_CONTENT_PATH);

    if file_path.exists() {
        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    } else {
        handle_init_safe(file_path);
    }

    add_line_to_file(file_path, "new line added");

    Ok(())
}
