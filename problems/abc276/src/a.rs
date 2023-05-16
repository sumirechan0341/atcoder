use proconio::input;

pub fn main() {
  input! {
    s: String
  }
  println!("{}", s.rfind('a').map(|i| (i + 1) as i32).unwrap_or(-1));
}