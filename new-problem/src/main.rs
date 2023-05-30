use std::process::Command;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::{Read, Write, BufWriter};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    if Path::new(input).exists() {
        println!("already exists problem!!");
        return;
    }

    let mut file = OpenOptions::new().read(true).open("./Cargo.toml").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    file.read_to_string(&mut file_content).unwrap();

    file = OpenOptions::new().write(true).truncate(true).open("./Cargo.toml").unwrap();
    let mut writer = BufWriter::new(file);
    let mut next = false;
    file_content.split("\n").for_each(|line| {
        if next {
            next = false;
            writer.write_all(format!("  \"{}\",", input).as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();
        }
        if line.contains("members = [") {
            next = true;
        }
        writer.write_all(line.as_bytes()).unwrap();
        if line != "]" {
            writer.write_all("\n".as_bytes()).unwrap();
        }
    });
    writer.flush().unwrap();
    Command::new("cmd").args(&["/C", &format!("cargo new --bin {} & cd {}", input, input)]).output().expect("failed to start `cargo new`");


    file = OpenOptions::new().read(true).open(format!("./{}/Cargo.toml", input)).unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    file.read_to_string(&mut file_content).unwrap();

    file = OpenOptions::new().write(true).truncate(true).open(format!("./{}/Cargo.toml", input)).unwrap();
    writer = BufWriter::new(file);
    next = false;
    file_content.split("\n").for_each(|line| {
        if next {
            next = false;
            writer.write_all("proconio = \"0.4.3\"\n".as_bytes()).unwrap();
            writer.write_all("itertools = \"0.10.5\"".as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();
        }
        if line.contains("[dependencies]") {
            next = true;
        }
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all("\n".as_bytes()).unwrap();
    });
    writer.flush().unwrap();
}