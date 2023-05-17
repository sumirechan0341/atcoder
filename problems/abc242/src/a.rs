use proconio::input;

pub fn main() {
    input !{
        a: f32,
        b: f32,
        c: f32,
        x: f32
    };
    if x <= a {
        println!("{}", 1.0f32);
        return
    }
    if x > b {
        println!("{}", 0.0f32);
        return
    }
    println!("{}", c / (b - a));
    
}