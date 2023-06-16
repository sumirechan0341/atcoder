use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut ans = vec![];
    for a in an {
        for i in 0..a {
            if (a - i) % 3 == 2 || (a - i) % 2 == 0 {
                continue;
            }
            ans.push(i);
            break;
        }
    }
    println!("{}", ans.iter().sum::<usize>());
}
