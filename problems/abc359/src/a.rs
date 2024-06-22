use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    println!(
        "{}",
        sn.iter().filter(|&s| *s == "Takahashi".to_string()).count()
    );
}
