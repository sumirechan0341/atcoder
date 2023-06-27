use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        k: usize,
        sn: [Chars; n]
    };
    let mut max = 0;
    for i in k..=n {
        for comb in (0..n).combinations(i) {
            let mut used = vec![0; 256];
            let mut local_count = 0;
            for j in comb {
                for c in &sn[j] {
                    used[*c as usize] += 1;
                }
            }
            for j in 0..256 {
                if used[j] == k {
                    local_count += 1;
                }
            }
            if local_count > max {
                max = local_count;
            }
        }
    }
    
    println!("{}", max);
}