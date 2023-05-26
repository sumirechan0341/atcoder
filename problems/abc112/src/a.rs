use proconio::input;

pub fn main() {
    input! {
        n: i32,
    };
    if n == 1 {
      println!("{}", "Hello World");
      return
    }
    input! {
      a: i32,
      b: i32
    };
    println!("{}", a + b );
}