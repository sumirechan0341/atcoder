use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        k: usize,
        q: usize,
        mut ak: [usize; k],
        lq: [usize; q]
    };
    
    for l in lq {
        if ak[l-1] != n {
            if l == k {
                // 一番右の駒でかつ一番右のマスにいないので動く
                ak[l-1] += 1;
            } else {
                if ak[l] != ak[l-1] + 1 {
                    ak[l-1] += 1;
                }
            }
        }
    }
    println!("{}", ak.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}