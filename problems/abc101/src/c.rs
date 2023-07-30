use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        an: [i32; n]
    };
    let mut lr = an.split(|&x| x == 1);
    let leftlen = lr.next().unwrap().len();
    let rightlen = lr.next().unwrap().len();
    let left = (leftlen + k-2)/(k-1);
    let right = (rightlen + k-2)/(k-1);
    let left_remain = left*(k-1) - leftlen;
    let right_remain = right*(k-1) - rightlen;
    println!("{}", if left_remain + right_remain >= k-1 { left + right - 1 } else { left + right });
}