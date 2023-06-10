use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        p: char,
        q: char
    };
    let line_num = vec![3, 1, 4, 1, 5, 9];
    let left = (p.min(q) as u8 - 65) as usize;
    let right = (p.max(q) as u8 - 65) as usize;

    println!("{}", line_num[left..right].iter().sum::<i32>());
}