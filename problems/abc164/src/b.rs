use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut a: i32,
        b: i32,
        mut c: i32,
        d: i32
    };
    for i in 0.. {
        if i % 2 == 0 {
            c -= b;
            if c <= 0 {
                println!("{}", "Yes");
                return;
            }
        } else {
            a -= d;
            if a <= 0 {
                println!("{}", "No");
                return;
            }
        }

    }
}