use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut abc: [u64; 3]
    };
    abc.sort();
    println!("{}", if abc[2] > abc[1] + abc[0] { (-1).to_string() } else { abc[2].to_string() });
}