use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        x: i32,
        am: [i32; m]
    };
    println!("{}", am.clone().into_iter().filter(|a| a > &x).collect::<Vec<_>>().len().min(am.into_iter().filter(|a| a < &x).collect::<Vec<_>>().len()));
}