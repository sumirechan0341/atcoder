use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    
    let keys = vec!['W', 'W', 'B', 'W', 'B', 'W', 'B', 'W', 'W', 'B', 'W', 'B'];
    for i in 0..12 {
        let mut ok = true;
        for j in 0..12 {
            if s[if (i+j)<20 { i+j } else { (i+j)-12 }] != keys[j] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", match i {
                0 => "Mi",
                11 => "Fa",
                9 => "So",
                7 => "La",
                5 => "Si",
                4 => "Do",
                2 => "Re",
                _ => "unreachble"
            });
            return;
        }
    }
}