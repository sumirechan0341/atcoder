use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        ak: [i64; k],
    };
    let mut lsn = vec![0; k / 2 + 1];
    let mut rsn = vec![0; k / 2 + 1];
    for i in (0..k / 2) {
        lsn[i + 1] = ak[i * 2 + 1] - ak[i * 2] + lsn[i];
        rsn[i + 1] = ak[k - i * 2 - 1] - ak[k - i * 2 - 2] + rsn[i];
    }

    let mut ans = std::i64::MAX;
    if k % 2 == 0 {
        let mut local = 0;
        for i in 0..k / 2 {
            local += ak[i * 2 + 1] - ak[i * 2];
        }
        ans = local;
    } else {
        for i in 0..k {
            if lsn[i / 2] + rsn[(k - i) / 2] < ans {
                ans = lsn[i / 2] + rsn[(k - i) / 2];
            }
        }
    }
    println!("{}", ans);
}
