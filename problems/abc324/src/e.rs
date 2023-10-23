use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: Chars,
        sn: [Chars; n]
    };
    // 先頭から一致する部分文字列を探す
    let mut prefix = vec![0; n];
    for i in 0..n {
        let mut cursor = 0;
        for j in 0..sn[i].len() {
            if cursor == t.len() {
                break;
            }
            if t[cursor] == sn[i][j] {
                cursor += 1;
            }
        }
        prefix[i] = cursor;
    }
    // 後方から一致する部分文字列を探す
    let mut postfix = vec![0; n];
    for i in 0..n {
        let mut cursor = t.len();
        for j in (0..sn[i].len()).rev() {
            if cursor == 0 {
                break;
            }
            if t[cursor-1] == sn[i][j] {
                cursor -= 1;
            }
        }
        postfix[i] = t.len()-cursor;
    }
    prefix.sort();
    postfix.sort();
    let mut cursor = n;
    let mut ans = 0;
    for i in 0..n {
        if cursor == 0 {
            ans += n-cursor;
            continue;
        }
        while prefix[i] + postfix[cursor-1] >= t.len() {
            cursor -= 1;
            if cursor == 0 {
                break;
            }
        }
        ans += n-cursor;
    }
    println!("{}", ans);
}