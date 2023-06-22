use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [usize; n]
    };
    // k回転した時の0..nの満足度を知りたい O(N^2)
    // 各piについて何回回転したとき満足度を得られるか、
    // このような数はちょうど3つあるが、これを走査すれば最初の情報と同じものがとれる
    // 2重ループを(n, n)の直積にしてループを回せば十分のパターンもある。
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[(pn[i]+n-1-i)%n] += 1;
        ans[(pn[i]+n-i)%n] += 1;
        ans[(pn[i]+n+1-i)%n] += 1;
    }
    ans.sort();
    println!("{}", ans.last().unwrap());
}