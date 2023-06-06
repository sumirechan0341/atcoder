use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        p: i32,
        an: [i32; n]
    };
    println!("{}", an.into_iter().filter(|x| x < &p).collect::<Vec<_>>().len());
}