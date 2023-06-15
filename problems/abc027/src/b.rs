use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let total = an.iter().sum::<usize>();
    if total % n != 0 {
        println!("{}", -1);
        return;
    }
    let target = total / n;
    let mut count = 0;
    let mut local_sum = 0;
    let mut local_target = target;
    for a in an {
        local_sum += a;
        if local_sum != local_target {
            local_target += target;
            count += 1;
        } else {
            local_target = target;
            local_sum = 0;
        }
    }
    println!("{}", count);
}