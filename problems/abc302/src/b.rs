use proconio::{input, marker::Chars};
pub fn main() {
    input !{
        h: usize,
        w: usize,
        sh: [Chars; h]   
    };
    let direction: [[i32; 2]; 8] = [[0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1], [1, 0], [1, 1]];
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            for d in &direction {
                let mut flag = false;
                for k in 0..5 {
                    if i as i32 + k*d[0] < 0 || i as i32 + k*d[0] > (h as i32 - 1) || j as i32 + k*d[1] < 0 || j as i32 +k*d[1] > (w as i32 -1) {
                        flag = true;
                        break;
                    }
                    
                    if sh[(i as i32 +(k*d[0])) as usize][(j as i32 +(k*d[1])) as usize] != snuke[k as usize] {
                        flag = true;
                        break;
                    }             
                }
                if flag {
                    continue;
                }
                for k in 0..5 {
                    println!("{} {}", (1+i as i32 +(k*d[0])) as usize, (1+j as i32 +(k*d[1])) as usize); 
                }
                return;
            }
        }
    }
}