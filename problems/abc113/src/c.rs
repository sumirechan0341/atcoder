use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        pym: [(usize, usize); m]
    };
    let mut rank = vec![0; m];
    let mut ipym = pym.iter().enumerate().collect::<Vec<_>>();
    ipym.sort_by_key(|x| x.1);
    let mut count = 1;
    let mut prev_pref = 100001;
    for (i, (p, y)) in ipym {
        if prev_pref == *p {
            count += 1;
        } else {
            count = 1;
        }
        rank[i] = count;
        prev_pref = *p;
    }
    for i in 0..m {
        println!("{:06}{:06}", pym[i].0, rank[i]);
    }
}