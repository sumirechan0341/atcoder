use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    println!(
        "{}",
        if s.iter().find_position(|&c| *c == 'R').unwrap()
            < s.iter().find_position(|&c| *c == 'M').unwrap()
        {
            "Yes"
        } else {
            "No"
        }
    )
}
