use proconio::input;

pub fn main() {
    input! {
      a: u32,
      b: u32,
      c: u32,
      d: u32,
      e: u32,
      f: u32,
      x: u32
    };
    let t = x / (a + c) * a * b + (x % (a + c)).min(a) * b;
    let a = x / (d + f) * d * e + (x % (d + f)).min(d) * e;
    println!("{}", if t == a {"Draw"} else if t > a {"Takahashi"} else { "Aoki" });
}