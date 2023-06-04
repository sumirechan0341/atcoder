use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        a: [Chars; n]
    };
    let direction = vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];
    let mut max = 0_i64;
    for i in 0..n {
        for j in 0..n {
            for d in &direction {
                let mut result = 0_i64;
                for k in 0..n {
                    let ln = n as i64;
                    let li = i as i64;
                    let lj = j as i64;
                    let lk = k as i64;
                    let x = (ln+li+lk*d.0) as usize % n;
                    let y = (ln+lj+lk*d.1) as usize % n;
                    result += a[x][y].to_digit(10).unwrap() as i64 * 10_i64.pow((n-k-1) as u32);
                }
                if max < result {
                    max = result;
                }
            }
        }
    }
    println!("{}", max);
}