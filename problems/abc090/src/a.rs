use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        c: [Chars; 3]
    };
    for i in 0..3 {
        print!("{}", c[i][i]);
    }
    println!("");
}