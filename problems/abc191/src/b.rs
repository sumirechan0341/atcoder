use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        x: i32,
        an: [i32; n]
    };
    let mut ans = vec![];
    for a in an {
        if a != x {
            ans.push(a);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}