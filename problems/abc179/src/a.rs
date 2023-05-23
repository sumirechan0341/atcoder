use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        s: Chars
    };
    println!("{}", if s[s.len()-1] == 's' { s.iter().collect::<String>() + "es" } else { s.iter().collect::<String>() + "s" });
}