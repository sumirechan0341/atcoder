use proconio::input;

pub fn main() {
  input! {
    n: usize,
    an: [u8; n]
  };
  let ans = an.iter().filter(|&a| a % 2 == 0).map(|e| e.to_string()).collect::<Vec<_>>();
  println!("{}", ans.join(" "));
}
