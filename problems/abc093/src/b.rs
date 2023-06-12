use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: i32,
        b: i32,
        k: i32
    };
    let mut ans = vec![];
    for i in a..(a+k).min(b+1) {
        ans.push(i);
    }
    for i in (b-k+1).max(a)..b+1 {
        if !ans.contains(&i) {
            ans.push(i);
        }
    }
    for an in ans {
        println!("{}", an);
    }
}