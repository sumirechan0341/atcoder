use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut dn: [usize; n]
    };
    for i in 0..n {
        dn[i] %= a + b;
    }
    dn.sort();
    dn.dedup();
    if dn.len() == 1 {
        println!("{}", "Yes");
        return;
    }
    dn.push(dn[0] + a + b);
    for i in 0..dn.len() - 1 {
        if (dn[i + 1] - dn[i]) % (a + b) > b {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
