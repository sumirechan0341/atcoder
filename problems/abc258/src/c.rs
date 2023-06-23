use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        qq: [(i32, usize); q]
    };
    let mut cursor = 0;
    for q in qq {
        match q {
            (1, x) => {
                cursor += x;
                cursor %= n;
            },
            (2, x) => {
                println!("{}", s[(x+n-cursor-1)%n]);  
            },
            _ => {
                println!("{}", "unreachable");
            }
        }
    }
}