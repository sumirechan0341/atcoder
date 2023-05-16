use proconio::input;

pub fn main() {
  input! {
    n: usize,
    hn: [usize; n]
  }
  println!("{}", hn.iter().enumerate().fold(0, |acc, (index, x)| if (hn[acc] as usize) < *x { index } else { acc } ) + 1);
}