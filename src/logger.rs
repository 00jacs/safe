use colored::*;

/// Prints a debug message (level: 4) in the same line
pub fn debugsn(message: &str) {
    print!("{}", message.bright_purple());
}

/// Prints a debug message (level: 4) in a new line
pub fn debug(message: &str) {
    println!("{}", message.bright_purple());
}

/// Prints a info message (level: 3) in the same line
pub fn infosn(message: &str) {
    print!("{}", message.normal());
}

/// Prints an info message (level: 3) in a new line
pub fn info(message: &str) {
    println!("{}", message.normal());
}

/// Prints a warn message (level: 2) in the same line
pub fn warnsn(message: &str) {
    print!("{}", message.yellow());
}

/// Prints an warn message (level: 2) in a new line
pub fn warn(message: &str) {
    println!("{}", message.yellow());
}

/// Prints an error message (level: 1) in the same line
pub fn errorsn(message: &str) {
    print!("{}", message.red());
}

/// Prints an error message (level: 1) in a new line
pub fn error(message: &str) {
    println!("{}", message.red());
}

/// Prints a success message (level: 1) in the same line
pub fn successsn(message: &str) {
    print!("{}", message.green());
}

/// Prints an success message (level: 1) in a new line
pub fn success(message: &str) {
    println!("{}", message.green());
}

