use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize
    };
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..n {
        let mut v = vec![];
        for j in 0..i+1 {
            if j == 0 || j == i {
                v.push(1);
            } else {
                v.push(ans[i-1][j-1] + ans[i-1][j]);
            }
        }
        ans.push(v)
    }
    for line in ans {
        println!("{}", line.iter().map(|x| x.to_string()).collect::<VS>().join(" "));
    }
}