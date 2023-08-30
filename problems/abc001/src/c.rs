use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut deg: f32,
        dis: f32
    };
    deg /= 10.0;
    let dir = vec!["N", "NNE", "NE", "ENE", "E", "ESE" , "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW" , "NW", "NNW"];
    let d = dir[(((((deg-11.25)/22.5).floor() as i32)+1)%16) as usize];
    let wind = (dis*10.0/60.0).round()/10.0;
    let w = if 0.0 <= wind && wind <= 0.2 {
        0
    } else if 0.3 <= wind && wind <= 1.5 {
        1
    } else if 1.6 <= wind && wind <= 3.3 {
        2
    } else if 3.4 <= wind && wind <= 5.4 {
        3
    } else if 5.5 <= wind && wind <= 7.9 {
        4
    } else if 8.0 <= wind && wind <= 10.7 {
        5
    } else if 10.8 <= wind && wind <= 13.8 {
        6
    } else if 13.9 <= wind && wind <= 17.1 {
        7
    } else if 17.2 <= wind && wind <= 20.7 {
        8
    } else if 20.8 <= wind && wind <= 24.4 {
        9
    } else if 24.5 <= wind && wind <= 28.4 {
        10
    } else if 28.5 <= wind && wind <= 32.6 {
        11
    } else {
        12
    };
    if w == 0 {
        println!("C 0");
    } else {
        println!("{} {}", d, w);
    }
}