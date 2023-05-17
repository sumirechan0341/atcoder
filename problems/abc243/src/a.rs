use proconio::input;

pub fn main() {
    input !{
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    };
    v = v % (a + b + c);
    if v - a < 0 {
        println!("{}", "F");
        return
    }
    v -= a;
    if v - b < 0 {
        println!("{}", "M");
        return
    }
    println!("{}", "T");
}