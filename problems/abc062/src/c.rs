use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h: i64,
        w: i64
    };
    if h%3 == 0 || w%3 == 0 {
        println!("{}", 0);
        return;
    }
    let mut min = std::i64::MAX;
    let total = h*w;
    for i in 1..=w {
        if (total - i*h)%2 == 0 {
            if min > (i*h-(total - i*h)/2).abs() {
                min = (i*h-(total - i*h)/2).abs();
            }
        } else {
            let local_min = (w-i)*(h/2);
            let local_max = (w-i)*(h/2+1);
            if min > ((i*h).max(local_max) - local_min.min(i*h)).abs() {
                min = ((i*h).max(local_max) - local_min.min(i*h)).abs();
            }
        }
    }
    for i in 1..=h {
        if (total - i*w)%2 == 0 {
            if min > (i*w-(total - i*w)/2).abs() {
                min = (i*w-(total - i*w)/2).abs();
            }
        } else {
            let local_min = (h-i)*(w/2);
            let local_max = (h-i)*(w/2+1);
            if min > ((i*w).max(local_max) - local_min.min(i*w)).abs() {
                min = ((i*w).max(local_max) - local_min.min(i*w)).abs();
            }
        }
    }
    println!("{}", min);
}