use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut n: i32,
        k: usize
    };
    for i in 0..k {
        n = f(n);
    }
    println!("{}", n);
}
fn f(x: i32) -> i32 {
    let mut s = x.to_string().chars().collect::<Vec<_>>();
    s.sort();
    let min = s.iter().collect::<String>().parse::<i32>().unwrap();
    s.reverse();
    let max = s.iter().collect::<String>().parse::<i32>().unwrap();
    return max-min
}