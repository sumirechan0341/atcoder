use proconio::input;

pub fn main() {
  input! {
    n: u8,
    s: String
  };
  for i in 1..n {
    if s.chars().nth((i - 1) as usize).unwrap() == s.chars().nth(i as usize).unwrap() {
      println!("No");
      return;
    }
  }
  println!("Yes");
}
