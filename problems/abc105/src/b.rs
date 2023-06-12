use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32,
    };
    match n {
        1 | 2 | 3 | 5 | 6 | 9 | 10 | 13 | 17 => {
            println!("{}", "No");
        },
        _ => {
            println!("{}", "Yes");
        }
    }
}