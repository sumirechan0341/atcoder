use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        ngs: [usize; 3]
    };
    if ngs.contains(&n) {
        println!("{}", "NO");
        return;
    }
    let mut check = vec![false; n+1];
    check[0] = true;
    for i in 0..100 {
        for j in (0..n).rev() {
            if check[j] {
                if j+1 <= n {
                    check[j+1] = true;
                }
                if j+2 <= n {
                    check[j+2] = true;
                }
                if j+3 <= n {
                    check[j+3] = true;
                }
            }
        }
        for &ng in &ngs {
            if ng <= n {
                check[ng] = false;
            }
        }
    }
    println!("{}", if check[n] { "YES" } else { "NO" });
}