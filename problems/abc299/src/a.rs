use proconio::input;

pub fn main() {
    input! {
        _n: u8,
        s: String
    }
    
    if s.split('|').nth(1).unwrap().contains('*') {
      println!("in");
    } else {
      println!("out");
    }
}