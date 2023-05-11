use std::process::Command;
use std::fs::OpenOptions;
use std::io::{Read, Write, BufWriter};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    let mut file = OpenOptions::new().read(true).write(true).open("./Cargo.toml").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    file.read_to_string(&mut file_content).unwrap();
    let mut writer = BufWriter::new(file);
    let mut next = false;
    file_content.split("\n").for_each(|line| {
        if next {
            next = false;
            writer.write_all(format!("  \"{}\"", input).as_bytes()).unwrap();
        }
        if line.contains("members = [") {
            next = true;
        }
        writer.write_all(line.as_bytes()).unwrap();
    });
    writer.flush().unwrap();
    println!("cargo new --bin {}", input);
    Command::new("cmd").args(&["/C", &format!("cargo new --bin {}", input)]).output().expect("failed to start `cargo new`");

}