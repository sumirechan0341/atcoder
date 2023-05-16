use proconio::input;

pub fn main() {
  input! {
    n: usize,
    x: usize,
    pn: [usize; n]
  }
  for i in 0..n {
    if pn[i] == x {
      println!("{}", i + 1);
      return;
    }
  }
}