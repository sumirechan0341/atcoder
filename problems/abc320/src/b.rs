use proconio::{input, marker::Chars, input_interactive};

pub fn main() {
    input! {
        s: Chars
    };
    let mut max = 0;
    for i in 0..s.len() {
        for j in i..s.len() {
            let mut ss = s[i..=j].to_vec();
            ss.reverse();
            if s[i..=j] == ss && max < j-i+1 {
                max = j-i+1;
            }
        }
    }
    println!("{}", max);
}