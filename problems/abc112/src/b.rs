use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        t: i32,
        mut ctn: [(i32, i32); n]
    };
    let mut ctn2 = ctn.into_iter().filter(|x| x.1 <= t).collect::<Vec<_>>();
    if ctn2.len() == 0 {
        println!("{}", "TLE");
        return;
    }
    ctn2.sort_by_key(|x| x.0);
    println!("{}", ctn2[0].0);
}