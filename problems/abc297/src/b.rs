use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut b_index = vec![];
    let mut r_flag = false;
    for (index, c) in s.iter().enumerate() {
        if c == &'B' {
            b_index.push(index);
        }
        if c == &'R' {
            r_flag = !r_flag;
        }
        if c == &'K' {
            if !r_flag {
                println!("{}", "No");
                return;
            }
        }
    }
    if b_index[0] % 2 ^ b_index[1] % 2 == 0 {
        println!("{}", "No");
        return;
    }
    println!("{}", "Yes");
}