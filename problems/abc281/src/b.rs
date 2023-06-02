use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    if s.len() != 8 {
        println!("{}", "No");
        return;
    }
    if !s[0].is_uppercase() {
        println!("{}", "No");
        return;
    }
    if !s[s.len()-1].is_uppercase() {
        println!("{}", "No");
        return;
    }
    match s[1..s.len()-1].iter().collect::<String>().parse::<i32>() {
        Ok(i) => {
            if 100000 <= i && i <= 999999 {
                println!("{}", "Yes");
                return;
            } else {
                println!("{}", "No");
                return;
            }
        },
        _ => {
            println!("{}", "No");
            return;
        }
    }
}
