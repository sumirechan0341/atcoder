use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        x: i128,
        k: i128,
        d: i128
    };
    if k*d <= x.abs() {
        println!("{}", x.abs() - k*d);
        return;
    }
    let remain = k - (x.abs()/d);
    if remain % 2 == 0 {
        println!("{}", x.abs() - (x.abs()/d) * d);
        return;
    } else {
        println!("{}", (x.abs() - (x.abs()/d + 1) * d).abs());
        return;
    }
}