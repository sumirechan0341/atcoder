use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        x: i32,
        mut mn: [i32; n]
    };
    mn.sort();
    let remain = x - mn.clone().into_iter().sum::<i32>();
    println!("{}", n as i32 + remain / mn[0]);
}