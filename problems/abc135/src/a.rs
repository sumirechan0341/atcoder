use proconio::input;

pub fn main() {
    input! {
        a: u32,
        b: u32
    };
    println!("{}", if (a + b) % 2 == 0 { ((a + b) / 2).to_string() } else { "IMPOSSIBLE".to_string() });
}