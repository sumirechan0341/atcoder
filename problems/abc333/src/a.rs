use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    println!(
        "{}",
        n.to_string().repeat(10).chars().take(n).collect::<String>()
    );
}
