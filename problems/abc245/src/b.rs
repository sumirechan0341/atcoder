use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    for i in 0..=2000 {
        if !an.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}