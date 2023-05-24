use proconio::input;

pub fn main() {
    input! {
        xn: [i32; 5]
    };
    for (index, &x) in xn.iter().enumerate() {
      if x == 0 {
        println!("{}", index + 1);
        return;
      }
    }
}