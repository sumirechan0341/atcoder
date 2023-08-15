use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut max = -2501;
    for i in 0..n {
        let mut local_atotal_max = -2501;
        let mut ttotal_when_atotal_max = -2501;
        for j in 0..n {
            let mut local_atotal = 0;
            let mut local_ttotal = 0;
            if i == j {
                continue;
            }
            let t = &an[i.min(j)..=i.max(j)];
            for k in 0..t.len() {
                if k%2 == 0 {
                    local_ttotal += t[k];
                } else {
                    local_atotal += t[k];
                }
            }
            if local_atotal_max < local_atotal {
                local_atotal_max = local_atotal;
                ttotal_when_atotal_max = local_ttotal;
            }
        }
        if ttotal_when_atotal_max > max {
            max = ttotal_when_atotal_max;
        }
    }
    println!("{}", max);
}