use proconio::input;

pub fn main() {
  input! {
    s: String
  };
  for (i, c) in s.chars().enumerate() {
    if c.is_uppercase() {
      println!("{}", i + 1);
    }
  }
}