use proconio::input;

pub fn main() {
    input! {
        s: String
    };
    let days = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    for i in 0..7 {
      if days[i] == &s {
        println!("{}", 7 - i);
        return
      } 
    }
}