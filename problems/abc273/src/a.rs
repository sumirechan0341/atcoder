use proconio::input;

pub fn main() {
  input! {
    n: u32
  }
  let mut ans = 1;
  for i in 1..=n {
    ans *= i;
  }
  println!("{}", ans);
}