use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        wn: [i32; n]
    };
    let mut min = 10000;
    for t in 1..n {
        let mut s1 = 0;
        let mut s2 = 0;
        for i in 0..t {
            s1 += wn[i];
        }
        for i in t..n {
            s2 += wn[i];
        }
        if (s1-s2).abs() < min {
            min = (s1-s2).abs()
        }
    }
    println!("{}", min);
}