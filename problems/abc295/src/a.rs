use proconio::input;

pub fn main() {
  input! {
    n: usize,
    wn: [String; n]
  };
  if wn.iter().find(|&w| w == "and" || w == "not" || w == "that" || w == "the" || w == "you").is_some() {
    println!("Yes");
  } else {
    println!("No");
  }
}
