use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        ss: [Chars; 3]
    };
    for s in ss {
        print!("{}", s[0].to_uppercase());
    }
    println!("{}", "");
}