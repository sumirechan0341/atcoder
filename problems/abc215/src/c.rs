use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [i32; n],
        tn: [i32; n]
    };
    let mut sortedtn = tn.iter().enumerate().collect::<Vec<_>>();
    sortedtn.sort_by_key(|x| x.1);
    let start = sortedtn[0].0;
    let mut ans = vec![std::i32::MAX; n];
    ans[start] = tn[start];
    for i in 1..n {
        ans[(start+i)%n] = tn[(start+i)%n].min(ans[(start+i-1)%n]+sn[(start+i-1)%n])
    }
    for a in ans {
        println!("{}", a);
    }
}