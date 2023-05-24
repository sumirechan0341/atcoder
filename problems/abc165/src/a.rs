use proconio::input;

pub fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32
    };
    for i in 0..=b {
        if a <= i && i <= b && i % k == 0 {
          println!("{}", "OK");
          return;
        }
    }
    println!("{}", "NG");
}