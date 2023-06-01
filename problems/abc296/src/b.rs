use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: [Chars; 8]
    };
    let row = ['8', '7', '6', '5', '4', '3', '2', '1'];
    let column = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", column[j], row[i]);
                return;
            }
        }
    }
}