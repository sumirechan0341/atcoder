use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        t: usize,
        ast: [(i64, i64); t]
    };
    for (a, s) in ast {
        if (s-a) >= 0 && (s-a) & a == a {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}