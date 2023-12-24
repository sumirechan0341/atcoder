use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        txn: [(i32, usize); n]
    };
    let mut potion = vec![0; n + 1];
    let mut k = 0;
    let mut ans = vec![];
    for i in (0..n).rev() {
        let (t, x) = txn[i];
        if t == 2 {
            potion[x] += 1;
            if k > 0 {
                k -= 1;
            }
        } else {
            if potion[x] > 0 {
                potion[x] -= 1;
                k += 1;
                ans.push("1");
            } else {
                ans.push("0");
            }
        }
    }
    ans.reverse();
    if potion.iter().any(|x| x > &0) {
        println!("-1");
        return;
    }
    println!("{}", k);
    println!("{}", ans.join(" "));
}
