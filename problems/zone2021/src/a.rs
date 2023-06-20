use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut count = 0;
    for i in 0..9 {
        if s[i..i+4].iter().collect::<String>() == "ZONe".to_string() {
            count += 1;
        }
    }
    println!("{}", count);
}