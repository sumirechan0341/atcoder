use proconio::input;

pub fn main() {
  input! {
    a: f32,
    b: f32
  }
  println!("{:.3}", b/a);
}