use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: Chars
    };
    if x.iter().min() == x.iter().max() {
        println!("{}", "Weak");
        return;
    }
    let mut prev = x[0].to_digit(10).unwrap();
    for i in 1..4 {
        let d = x[i].to_digit(10).unwrap();
        if (prev + 1) % 10 == d {
            prev = d;
        } else {
            println!("{}", "Strong");
            return
        }
    }
    println!("{}", "Weak");
}