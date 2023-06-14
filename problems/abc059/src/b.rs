use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: Chars,
        b: Chars
    };
    if a.len() > b.len() {
        println!("{}", "GREATER");
        return;
    }
    if b.len() > a.len() {
        println!("{}", "LESS");
        return;
    }
    for i in 0..a.len() {
        if a[i] > b[i] {
            println!("{}", "GREATER");
            return;
        }
        if b[i] > a[i] {
            println!("{}", "LESS");
            return;
        }
    }
    println!("{}", "EQUAL");
}