use proconio::input;

pub fn main() {
    input! {
        t: u32
    };
    println!("{}", f(f(f(t) + t) + f(f(t))));
}

fn f(x: u32) -> u32 {
  x.pow(2) + x * 2 + 3
}