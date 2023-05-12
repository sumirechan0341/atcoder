use proconio::input;

pub fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        _s: usize,
        mut an: [usize; n]
    }
    for i in 0..=q-p {
        let tmp = an[p + i - 1];
        an[p + i - 1] = an[r + i - 1];
        an[r + i - 1] = tmp;
    }
    println!("{}", an.iter().map(|&a| a.to_string()).collect::<Vec<String>>().join(" "));
}