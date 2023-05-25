use proconio::input;

pub fn main() {
    input! {
        s: String
    };
    let weathers = [ "Sunny", "Cloudy", "Rainy" ];
    for i in 0..3 {
      if weathers[i] == &s {
        println!("{}", weathers[(i + 1) % 3]);
        return
      }
    }
}