use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut s: Chars
    };
    if s[0] != 'A' {
        println!("{}", "WA");
        return;
    }
    let mut count = 0;
    let mut index = 0;
    for i in 2..s.len()-1 {
        if s[i] == 'C' {
            count += 1;
            index = i;
        }
    }
    if count != 1 {
        println!("{}" , "WA");
        return;
    }
    s.remove(index);
    s.remove(0);
    if s.into_iter().any(|c| c.is_uppercase()) {
        println!("{}", "WA");
        return;
    }
    println!("{}", "AC");
}