use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        o: Chars,
        e: Chars
    };
    for i in 0..o.len() {
        print!("{}", o[i]);
        if i == e.len() {
            break;
        }
        print!("{}", e[i]);
    }
    println!("{}", "");
}