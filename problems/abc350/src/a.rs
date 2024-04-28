use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: String
    };
    println!(
        "{}",
        if s <= "ABC349".to_string() && s != "ABC316".to_string() && s != "ABC000".to_string() {
            "Yes"
        } else {
            "No"
        }
    );
}
