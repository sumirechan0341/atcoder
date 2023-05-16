use proconio::input;

pub fn main() {
  input! {
    n: usize,
    an: [u32; n]
  }
  println!("{}", an.iter().sum::<u32>())
}