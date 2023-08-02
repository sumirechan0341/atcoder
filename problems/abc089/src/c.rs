use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let marchs = vec!['M', 'A', 'R', 'C', 'H'].iter().map(|c| sn.iter().filter(|&x| x.starts_with(*c)).count()).collect::<Vec<_>>();
    let mut total = 0;
    for i in 0..5 {
        for j in i+1..5 {
            for k in j+1..5 {
                total += marchs[i] * marchs[j] * marchs[k];
            }
        }
    }
    println!("{}", total);
}