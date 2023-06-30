use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut abn: [(f64, f64); n]
    };
    let mut head = 0;
    let mut tail = n-1;
    let mut from_left = 0.0;
    while head != tail {
        if abn[head].0 / abn[head].1 == abn[tail].0 / abn[tail].1 {
            from_left += abn[head].0;
            if tail - head == 1 {
                println!("{}", from_left);
                return;
            }
            head += 1;
            tail -= 1;
        } else if abn[head].0 / abn[head].1 > abn[tail].0 / abn[tail].1 {
            abn[head].0 -= abn[tail].0 / abn[tail].1 * abn[head].1;
            from_left += abn[tail].0 / abn[tail].1 * abn[head].1;
            tail -= 1;
        } else {
            abn[tail].0 -= abn[head].0 / abn[head].1 * abn[tail].1;
            from_left += abn[head].0;
            head += 1;

        }
    }
    from_left += abn[head].0 / 2.0;
    println!("{}", from_left);
}