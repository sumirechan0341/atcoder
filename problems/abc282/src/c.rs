use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        s: Chars
    };
    let mut mode = true;
    for i in 0..n {
        if s[i] == '"' {
            mode = !mode;
        }
        print!("{}", if s[i] == ',' && mode { '.' } else { s[i] });
    }
    println!("{}", "");
}