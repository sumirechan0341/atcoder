use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    println!("{}", if an.into_iter().filter(|x| x % 2 == 0 && (x % 3 != 0 && x % 5 != 0)).collect::<Vec<_>>().len() > 0 { "DENIED" } else { "APPROVED" });
}