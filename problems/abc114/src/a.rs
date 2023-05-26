use proconio::input;

pub fn main() {
    input! {
        x: i32
    };
    println!("{}", match x {
      7 | 5 | 3 => "YES",
      _ => "NO"
    });
}