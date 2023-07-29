use proconio::{input, marker::Chars};
 
pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let mut b_start: i64 = -20000000000;
    let mut b_end: i64 = 20000000000;
    let mut ans = std::i64::MAX;
    while b_end - b_start > 1 {
        let mut abs_local = 0;
        let mut local_bs = 0;
        let mut local_be = 0;
        let now_b = ((b_start + b_end) as f64 / 2.0).floor() as i64;
        for i in 0..n {
            local_bs += (an[i] - (b_start + i as i64 +1)).abs();
            local_be += (an[i] - (b_end + i as i64 +1)).abs();
            abs_local += (an[i] - (now_b + i as i64 +1)).abs();
        }
        if abs_local < ans {
            ans = abs_local;
        }
        // println!("(bs {}, b_start {}) (be {}, b_end {}) (b_now {}, b_now {})", b_start, local_bs, b_end, local_be, now_b, abs_local);
        if (abs_local - local_bs).abs() > (abs_local - local_be).abs() {
            b_start = now_b;
        } else {
            b_end = now_b;
        }
        // println!("now_b {} local {} start {} end {}", now_b, local, b_start, b_end);
    }
    println!("{}", ans);
    // let mut kotae = 10000;
    // let mut my_b = 10000;
    // for b in -10..10 {
    //     let mut abs_local = 0;
    //     for i in 0..n {
    //         abs_local += (an[i] - (b + i as i64 +1)).abs();
    //     }
    //     if abs_local < kotae {
    //         my_b = b;
    //         kotae = abs_local;
    //     }
    // }
    // println!("b {} kotae {}", my_b, kotae);
    // println!("b_start {} b_end {} ans {}", b_start, b_end, ans);
}