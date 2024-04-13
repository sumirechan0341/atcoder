use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    for i in 1..=n {
        if i % 3 == 0 {
            print!("{}", "x");
        } else {
            print!("{}", "o");
        }
    }
    println!("{}", "");
}
