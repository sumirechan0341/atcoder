use proconio::input;

pub fn main() {
    input! {
        s: u32,
        t: u32,
        x: u32,
    };
    if s < t {
      println!("{}", if s <= x && x < t { "Yes" } else { "No" });
    } else {
      println!("{}", if t <= x && x < s { "No" } else { "Yes" });
    }
}