use proconio::input;

pub fn main() {
  input! {
    n: usize,
    m: usize,
    an: [u8; n],
    bm: [u8; m]
  };
  let mut score: u32 = 0;

  for i in 0..n {
    if bm.contains(&((i as u8) + 1)) {
      score += an[i] as u32;
    }
  }
  println!("{}", score);
}