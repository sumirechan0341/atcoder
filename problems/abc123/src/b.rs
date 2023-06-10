use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        abcde: [i32; 5]
    };
    let mut dummy = abcde.clone().into_iter().map(|x| (10 - x % 10) % 10).collect::<Vec<_>>();
    dummy.sort();
    println!("{}", abcde.iter().sum::<i32>() + dummy[..4].into_iter().sum::<i32>());
}