use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: i128,
        x: i128,
        an: [i128; n]
    };
    let total = an.clone().iter().sum::<i128>();
    let disposable = an.clone().into_iter().map(|c| c/x).sum::<i128>();
    let mut change = an.into_iter().map(|c| c%x).collect::<Vec<_>>();
    change.sort();
    change.reverse();
    if k <= disposable {
        println!("{}", total-x*k);
        return;
    } else {
        let extra = change[0..((k-disposable) as usize).min(n)].iter().sum::<i128>();
        println!("{}", total-x*disposable-extra);
        return;
    }
}