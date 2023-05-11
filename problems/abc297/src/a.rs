use proconio::input;

pub fn main() {
    input! {
        n: usize,
        d: u32,
        tn: [u32; n]
    }
    
    for i in 0..n-1 {
      if tn[i+1] - tn[i] <= d {
        println!("{}", tn[i + 1]);
        return;
      }
    }
    println!("-1");
}