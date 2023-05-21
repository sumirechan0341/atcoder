use proconio::input;

pub fn main() {
    input! {
        mut abc: [i32; 3]
    };
    abc.sort();
    println!("{}", if abc[1] as f32 == (abc[0] as f32+ abc[2] as f32) / 2.0_f32 { "Yes" } else { "No" });
}