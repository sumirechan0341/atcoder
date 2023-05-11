use proconio::input;

pub fn main() {
    input! {
        n: u8,
        s: String
    }
    
    if s.contains('x') {
      println!("No");
      return;
    }
    if s.contains('o') {
      println!("Yes");
      return;
    }
    println!("No");
    return;
}