use std::collections::VecDeque;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: usize,
        s: Chars
    };
    let mut bits = VecDeque::new();
    for c in format!("{:b}", x).chars() {
        bits.push_back(c);
    }
    for i in 0..n {
        match s[i] {
            'U' => {
                bits.pop_back();
            }
            'L' => {
                bits.push_back('0');
            }
            'R' => {
                bits.push_back('1');
            }
            _ => println!("{}", "unreachable"),
        };
    }
    println!(
        "{}",
        usize::from_str_radix(&bits.iter().collect::<String>(), 2).unwrap()
    );
}
