use proconio::input;

pub fn main() {
    input! {
        d: i32
    };
    println!("{}", match d {
      25 => "Christmas",
      24 => "Christmas Eve",
      23 => "Christmas Eve Eve",
      _ => "Christmas Eve Eve Eve",
    });
}