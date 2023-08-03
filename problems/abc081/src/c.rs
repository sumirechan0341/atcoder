use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize, 
        k: usize,
        an: [usize; n]
    };
    let mut count = vec![0; 200001];
    let mut num = 0;
    for i in 0..n {
        if count[an[i]] == 0 {
            num += 1;
        }
        count[an[i]] += 1;
    }
    if num <= k {
        println!("{}", 0);
        return;
    }
    count.sort();
    println!("{}", count.iter().filter(|&x| *x != 0).take(num-k).sum::<i32>());
}