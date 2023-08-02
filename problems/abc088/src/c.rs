use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        c: [i32; 9]
    };
    let a1_min = c[0].min(c[1]).min(c[2]);
    let a2_min = c[3].min(c[4]).min(c[5]);
    let a3_min = c[6].min(c[7]).min(c[8]);

    for a1 in 0..=a1_min {
        for a2 in 0..=a2_min {
            for a3 in 0..=a3_min {
                let b1 = c[0]-a1;
                let b2 = c[1]-a1;
                let b3 = c[2]-a1;
                let a = vec![a1, a2, a3];
                let b = vec![b1, b2, b3];
                let mut ok = true;
                for i in 0..9 {
                    if c[i] != a[i/3]+b[i%3] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    println!("{}", "Yes");
                    return;
                }
            }
        }
    }
    println!("{}", "No");
}