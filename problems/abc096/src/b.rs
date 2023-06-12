use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut abc: [i32; 3],
        k: u32
    };
    abc.sort();
    println!("{}", abc[0] + abc[1] + abc[2]*2_i32.pow(k));
}