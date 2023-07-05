use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n],
        bn: [usize; n],
        mut cn: [usize; n]
    };
    let mut count: i64 = 0;
    let mut point = vec![0; n];
    for i in 0..n {
        point[bn[cn[i]-1]-1] += 1;
    }
    for i in 0..n {
        count += point[an[i]-1];
    }
    println!("{}", count);
}