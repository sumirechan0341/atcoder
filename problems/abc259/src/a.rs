use proconio::input;

pub fn main() {
    input! {
        _n: i32,
        m: i32,
        x: i32,
        t: i32,
        d: i32
    };
    if x <= m {
      println!("{}", t);
    } else {
      println!("{}", t + (m - x) * d);
    }
}