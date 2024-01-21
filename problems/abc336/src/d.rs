use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let mut ans = 1;
    let mut counter = 0;
    let mut back_counter = 0;
    let mut status = true;
    // println!("c {:?}", counter);
    // println!("bc {:?}", back_counter);

    for i in 0..n {
        if status && an[i] >= counter + 1 {
            counter += 1;
        } else {
            // 増加終了
            if counter - 1 <= back_counter {
                if ans < (counter + back_counter) / 2 + 1 {
                    ans = (counter + back_counter) / 2 + 1;
                }
                status = true;
                counter = 1;
                back_counter = 0;
            } else if an[i] < counter - back_counter - 1 {
                // つじつま合わせが無理な場合
                // 登りと下りで数が小さい方を採用
                if ans < (counter + back_counter) / 2 + 1 {
                    ans = (counter + back_counter) / 2 + 1;
                }
                status = true;
                counter = 1;
                back_counter = 0;
            } else {
                status = false;
                back_counter += 1;
            }
        }
        // println!("c {:?}", counter);
        // println!("bc {:?}", back_counter);
    }
    // println!("c {:?}", counter);
    // println!("bc {:?}", back_counter);
    if ans < (counter + back_counter) / 2 + 1 {
        ans = (counter + back_counter) / 2 + 1;
    }

    println!("{}", ans);
}
