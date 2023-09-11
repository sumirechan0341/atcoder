use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        c9: [i64; 9]
    };
    let mut count = 0;
    for p in (0..9).permutations(9) {
        let mut v = vec![0; 9];
        for i in 0..9 {
            if p[i] == 0 {
                // 横
                if v[1] > 0 && v[2] > 0 && v[1] == v[2] {
                    count += 1;
                    break;
                }
                // 縦
                if v[3] > 0 && v[6] > 0 && v[3] == v[6] {
                    count += 1;
                    break;
                }
                // ななめ
                if v[4] > 0 && v[8] > 0 && v[4] == v[8] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 1 {
                // 横
                if v[0] > 0 && v[2] > 0 && v[0] == v[2] {
                    count += 1;
                    break;
                }
                // 縦
                if v[4] > 0 && v[7] > 0 && v[4] == v[7] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 2 {
                // 横
                if v[0] > 0 && v[1] > 0 && v[0] == v[1] {
                    count += 1;
                    break;
                }
                // 縦
                if v[5] > 0 && v[8] > 0 && v[5] == v[8] {
                    count += 1;
                    break;
                }
                // ななめ
                if v[4] > 0 && v[6] > 0 && v[4] == v[6] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 3 {
                // 横
                if v[4] > 0 && v[5] > 0 && v[4] == v[5] {
                    count += 1;
                    break;
                }
                // 縦
                if v[0] > 0 && v[6] > 0 && v[0] == v[6] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 4 {
                // 横
                if v[3] > 0 && v[5] > 0 && v[3] == v[5] {
                    count += 1;
                    break;
                }
                // 縦
                if v[1] > 0 && v[7] > 0 && v[1] == v[7] {
                    count += 1;
                    break;
                }
                // ななめ
                if v[0] > 0 && v[8] > 0 && v[0] == v[8] {
                    count += 1;
                    break;
                }
                if v[2] > 0 && v[6] > 0 && v[2] == v[6] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 5 {
                // 横
                if v[3] > 0 && v[4] > 0 && v[3] == v[4] {
                    count += 1;
                    break;
                }
                // 縦
                if v[2] > 0 && v[8] > 0 && v[2] == v[8] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 6 {
                // 横
                if v[7] > 0 && v[8] > 0 && v[7] == v[8] {
                    count += 1;
                    break;
                }
                // 縦
                if v[0] > 0 && v[3] > 0 && v[0] == v[3] {
                    count += 1;
                    break;
                }
                // ななめ
                if v[4] > 0 && v[2] > 0 && v[4] == v[2] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 7 {
                // 横
                if v[6] > 0 && v[8] > 0 && v[6] == v[8] {
                    count += 1;
                    break;
                }
                // 縦
                if v[1] > 0 && v[4] > 0 && v[1] == v[4] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
            if p[i] == 8 {
                // 横
                if v[6] > 0 && v[7] > 0 && v[6] == v[7] {
                    count += 1;
                    break;
                }
                // 縦
                if v[2] > 0 && v[5] > 0 && v[2] == v[5] {
                    count += 1;
                    break;
                }
                // ななめ
                if v[0] > 0 && v[4] > 0 && v[0] == v[4] {
                    count += 1;
                    break;
                }
                v[p[i]] = c9[p[i]];
            }
        }
    }
    println!("{}", 1.0-(count as f64)/362880.0);
}