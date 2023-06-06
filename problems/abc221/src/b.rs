use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut s: Chars,
        t: Chars
    };
    if s == t {
        println!("{}", "Yes");
        return;
    }
    for i in 0..s.len()-1 {
        let mut temp = s.clone();
        temp[i] = temp[i+1];
        temp[i+1] = s[i];

        if temp == t {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}