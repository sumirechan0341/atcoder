use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        r: i64,
        an: [i64; n]
    };
    let mut sn = vec![0; n + 1];
    for i in 0..n {
        sn[i + 1] = sn[i] + an[i];
    }
    let mut x = 0;
    let mut y = 1;
    let mut z = 2;
    let mut w = 3;
    'out: while x <= n - 3 {
        while sn[y] - sn[x] < p {
            y += 1;
            if y > n {
                break 'out;
            }
        }
        if sn[y] - sn[x] != p {
            x += 1;
            continue;
        }

        while sn[z] - sn[y] < q {
            z += 1;
            if z > n {
                break 'out;
            }
        }
        if sn[z] - sn[y] != q {
            x += 1;
            continue;
        }

        while sn[w] - sn[z] < r {
            w += 1;
            if w > n {
                break 'out;
            }
        }
        if sn[w] - sn[z] != r {
            x += 1;
            continue;
        }
        println!("{}", "Yes");
        return;
    }
    println!("{}", "No");
}
