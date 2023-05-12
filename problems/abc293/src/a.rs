use proconio::input;

pub fn main() {
  input! {
    s: String
  };
  let mut ans = String::new();
  for i in 0..s.len()/2 {
    ans.push(s.chars().nth(2 * i + 1).unwrap());
    ans.push(s.chars().nth(2 * i).unwrap())
  }
  println!("{}", ans);
}