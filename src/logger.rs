use colored::*;

pub fn info(message: &str) {
    println!("{}", message.blue());
}

pub fn warn(message: &str) {
    println!("{}", message.yellow());
}

pub fn error(message: &str) {
    println!("{}", message.red());
}

pub fn success(message: &str) {
    println!("{}", message.green());
}
