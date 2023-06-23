use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u64,
        x: u64,
        y: u64
    };
    println!("{}", red_exchange(n, x, y, 1));

}
fn red_exchange(lv: u64, x: u64, y: u64, num: u64) -> u64 {
    if lv == 1 {
        return 0;
    }
    return red_exchange(lv-1, x, y, num) + blue_exchange(lv, x, y, num*x);
}

fn blue_exchange(lv: u64, x: u64, y: u64, num: u64) -> u64 {
    if lv == 1 {
        return num;
    }
    return red_exchange(lv-1, x, y, num) + blue_exchange(lv-1, x, y, num*y);
}