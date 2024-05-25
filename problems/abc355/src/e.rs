use proconio::{input, input_interactive, marker::Chars};

pub fn main() {
    input_interactive! {
        n: usize,
        mut l: usize,
        r: usize,
    };
    let mut ans = 0;
    loop {
        if l - 1 == r {
            println!("! {}", ans % 100);
            return;
        }
        if l & 1 == 1 {
            println!("? {} {}", 0, l);
            input_interactive! {
                t: i32
            }
            if t == -1 {
                return;
            }
            l += 1;
            ans += t;
        } else {
            let mut i = 1;
            loop {
                if l + 2_usize.pow(i + 1) > r {
                    break;
                }
                i += 1;
            }
            println!("? {} {}", i, l / 2_usize.pow(i));
            input_interactive! {
                t: i32
            }
            if t == -1 {
                return;
            }
            ans += t;
            l += 2_usize.pow(i);
        }
    }
}
