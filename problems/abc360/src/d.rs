use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: i64,
        s: Chars,
        xn: [i64; n]
    };

    let mut left = vec![];
    let mut right = vec![];

    for (i, &pos) in xn.iter().enumerate() {
        if s[i] == '0' {
            left.push(pos);
        } else {
            right.push(pos);
        }
    }

    left.sort();
    right.sort();

    let mut ans = 0;
    for &pos in &right {
        let max_pos = pos + 2 * t;
        let lower_bound = left.binary_search(&pos).unwrap_or_else(|x| x);
        let upper_bound = left.binary_search(&(max_pos + 1)).unwrap_or_else(|x| x);

        ans += (upper_bound - lower_bound) as i64;
    }

    println!("{}", ans);
}
