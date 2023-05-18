use proconio::input;

pub fn main() {
    input! {
        n: u32,
        k: u32,
        a: u32
    };
    println!("{}",if (k - 1 + a) % n == 0 { n } else { (k - 1 + a) % n });
}