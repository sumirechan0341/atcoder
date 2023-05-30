use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        mut abc: [i32; 3]
    };
    abc.sort();
    println!("{}", if abc[0] + abc[1] == abc[2] { "Yes" } else { "No" });
}