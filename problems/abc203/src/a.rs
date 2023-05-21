use proconio::input;

pub fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    let mut bucket: [usize; 7] = [0 ;7];
    bucket[a] += 1;
    bucket[b] += 1;
    bucket[c] += 1;
    if bucket.iter().filter(|&x| x == &1).count() == 3 {
      println!("{}", 0);
      return;
    }
    for i in 0..7 {
      if bucket[i] % 2 == 1 {
        println!("{}", i);
        return
      }
    }
}