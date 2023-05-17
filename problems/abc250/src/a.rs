use proconio::input;

pub fn main() {
    input! {
        h: u32,
        w: u32,
        r: u32,
        c: u32
    };
    let mut ans = 4;
    if h == 1 {
      ans -= 1;
    }
    if w == 1 {
      ans -= 1;
    }
    if r == 1 || r == h {
      ans -= 1;
    }
    if c == 1 || c == w {
      ans -= 1;
    }
    println!("{}", ans);
}