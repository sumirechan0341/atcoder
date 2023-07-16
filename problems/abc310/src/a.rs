use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        p: i32,
        q: i32,
        mut dn: [i32; n]
    };
    dn.sort();
    println!("{}", p.min(q+dn[0]));
}