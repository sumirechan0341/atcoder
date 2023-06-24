use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        a7n: [i32; 7*n]
    };
    let mut ans = vec![];
    for i in 0..n {
        let mut local = 0;
        for j in 0..7 {
            local += a7n[7*i+j];
        }
        ans.push(local);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}