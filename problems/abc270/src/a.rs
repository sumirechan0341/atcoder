use proconio::input;

pub fn main() {
  input! {
    a: u32,
    b: u32
  }
  println!("{}", a | b);
}