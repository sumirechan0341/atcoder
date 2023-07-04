use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32
    };
    if a > 0 {
        if b > 0 {
            println!("{}", if a > b { ">" } else if a == b { "=" } else { "<" });
        } else if b == 0 {
            println!("{}", ">");
        } else {
            if c % 2 == 1 {
                println!("{}", ">");
            } else {
                println!("{}", if a.abs() > b.abs() { ">" } else if a.abs() == b.abs() { "=" } else { "<" });
            }
        }
    } else if a == 0 {
        if b > 0 {
            println!("{}", "<");
        } else if b == 0 {
            println!("{}", "=");
        } else {
            if c % 2 == 1 {
                println!("{}", ">");
            } else {
                println!("{}", "<");
            }
        }
    } else {
        if b > 0 {
            if c % 2 == 1 {
                println!("{}", "<");
            } else {
                println!("{}", if a.abs() > b.abs() { ">" } else if a.abs() == b.abs() { "=" } else { "<" });
            }
        } else if b == 0 {
            if c % 2 == 1 {
                println!("{}", "<");
            } else {
                println!("{}", ">");
            }
        } else {
            if a == b {
                println!("{}", "=");
            } else if c % 2 == 1 {
                println!("{}", if a.abs() > b.abs() { "<" } else { ">" });
            } else {
                println!("{}", if a.abs() > b.abs() { ">" } else { "<" });
            }
        }
    }
}