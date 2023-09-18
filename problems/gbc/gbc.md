# 緑以下コンテストについて
https://mojacoder.app/users/kusirakusira/contests/GBC

## A
### 思考
N角形の内角の和を答える問題。

(n-2)*180
```rust
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    println!("{}", (n-2)*180);
}
```

## B
### 思考
H, Wのディスプレイとh, wの画像がある。
ディスプレイは縦向きと横向きにでき、画像が自動で縦か横にフィットするとき、
どちら向きに表示すると面積が大きくなるか答える問題。

Bからむずい…
よくわからないのでいろいろ実験して、(H > W) == (h > w) で判定できそうだったので投げたら通った。
```rust
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        hh: i32,
        ww: i32,
        h: i32,
        w: i32
    };
    if hh > ww && h > w {
        println!("{}", "Vertical");
    } else if hh > ww && h < w {
        println!("{}", "Horizontal");
    } else if hh < ww && h > w {
        println!("{}", "Horizontal");
    } else if hh < ww && h < w {
        println!("{}", "Vertical");
    } else {
        println!("{}", "Same");
    }
}
```

## C
### 思考
長さNの数列が与えられる。
広義単調増加な数列になるような順列の総数を求める問題。

数が同じ部分で順番の入れ替えの余地があり、
そのときの並べ方は、(同じ数の個数)! である。
ソートしたあとに、前方と数が同じかどうか判定することで、
同じ数の個数を数え計算した。
この手の集計、最後にループから脱出したときのことを毎回忘れる。
```rust
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n]
    };
    let p = 998244353;
    let mut ans = 1_i64;
    an.sort();
    let mut count = 1;
    let mut prev = an[0];
    for i in 1..n {
        if an[i] == prev {
            count += 1;
        } else {
            prev = an[i];
            ans *= fact_mod(count, p);
            ans %= p;
            count = 1;
        }
    }
    ans *= fact_mod(count, p);
    ans %= p;
    println!("{}", ans);
}

fn fact_mod(n: i64, p: i64) -> i64 {
    let mut res = 1;
    for i in 1..=n {
        res *= i;
        res %= p;
    }
    return res;
}
```

## D
### 思考
2つのN×Nの行列が与えられる。
片方を「転置」、「回転」、「上下反転」、「左右反転」することで、
もう片方に一致させることができるか判定する問題。

全部試せばよい。
転置は位数が2、回転は位数が4、上下反転は位数が2、左右反転は位数が2であるため、
全部の組み合わせを試しても十分間に合う。
全部の操作をもつライブラリを作るべき。
```rust
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        snn: [[char; n]; n],
        tnn: [[char; n]; n]
    };
    for i in 0..4 {
        let ss = rotate(&snn, i);
        for j in 0..2 {
            let sss = flip_h(&ss, j);
            for k in 0..2 {
                let ssss = flip_v(&sss, k);
                for l in 0..2 {
                    let sssss = transpose(&ssss, l);
                    if is_same(&sssss, &tnn) {
                        println!("{}", "Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("{}", "No");
}

fn rotate(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    let mut res = s.clone();
    for k in 0..n {
        let now = res.clone();
        for i in 0..s.len() {
            for j in 0..s.len() {
                res[i][j] = now[s.len()-1-j][i];
            }
        }
    }
    return res;
}

fn flip_h(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    if n%2 == 0 {
        return s.clone();
    }
    let mut res = vec![vec!['0'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            res[i][j] = s[s.len()-1-i][j];
        }
    }
    return res;
}

fn flip_v(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    if n%2 == 0 {
        return s.clone();
    }
    let mut res = vec![vec!['0'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            res[i][j] = s[i][s.len()-1-j];
        }
    }
    return res;
}

fn transpose(s: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    if n%2 == 0 {
        return s.clone();
    }
    let mut res = vec![vec!['0'; s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            res[i][j] = s[j][i];
        }
    }
    return res;
}

fn is_same(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    for i in 0..s.len() {
        for j in 0..s.len() {
            if s[i][j] != t[i][j] {
                return false;
            }
        }
    }
    return true;
}

```

## E
### 思考
(1, 2, .., N)の数列が与えられる。
自由に並び替えて、隣接した2項の和をとり、その数を記録していく。
記録された数の種類数が最も最小になるときの値を答える問題。

実験してみると、次のように並べるのが最適だと分かる。

1, N, 2, (N-1), ...

このとき(N+1)と(N+2)しか現れない。
これはNが偶数でも奇数でも同じである。
N=1のときのみ、1種類なので気を付ける。
面白い問題。

```rust
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    if n == 2 {
        println!("{}", 1);
    } else {
        println!("{}", 2);
    }
}
```

## F
### 思考
大人数じゃんけんを以下のルールで行う。
- 1人以上に出された手のうち、最小の手が1種類であった場合、その手を出した人が勝利する。
- 1人以上に出された手のうち、最小の手が2種類であった場合、普通のじゃんけんで強い方の手を出した人が勝利する。
N人の出す手が与えられ、大人数じゃんけんを行う対象が区間のクエリで与えられるので、
勝利できる手をすべて出力する問題。

じゃんけんする対象が区間のクエリで与えられ、その区間での手の数を集計すればいいので、累積和を使えばよい。
累積変数をsとすれば、[l, r]の各手の数は、s[r]-s[l-1]で求めることができる。
あとは、人数と自分の手を与えて、勝つかどうか判定する関数を頑張って実装すればよい。

自分の出した手がグーのときのみ説明する。
まず、明確に勝つ場合を書いて処理する。
(グーの数) <= (チョキの数) && (グーの数) < (パーの数)
のとき勝利できる。
グーとチョキが同数の場合は勝てるが、グーとパーが同数の場合に勝てないことに注意する。
0人の手があるとき、処理が変わるのでその場合分けを行う。
チョキが0人のとき、(グーの数) < (パーの数) のとき勝利できる。
パーが0人のとき、(グーの数) <= (チョキの数) のとき勝利できる。
グーが0人になることはない。
なぜならば少なくとも自分がグーを出しているからである。

これで判定できると思ったのだが、条件をひとつ見落としていた。
チョキの数が0でパーの数が0のときは、グーを出した人全員が勝利する。
これですべての条件が列挙でき、他の手も同様に考えれば解くことができる。

```rust
use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [char; n],
        q: usize,
        lrq: [(usize, usize); q]
    };
    let mut s = vec![vec![0; n+1]; 3];
    for i in 0..n {
        if an[i] == 'G' {
            s[0][i+1] = s[0][i]+1;
            s[1][i+1] = s[1][i];
            s[2][i+1] = s[2][i];
        } else if an[i] == 'C' {
            s[0][i+1] = s[0][i];
            s[1][i+1] = s[1][i]+1;
            s[2][i+1] = s[2][i];
        } else {
            s[0][i+1] = s[0][i];
            s[1][i+1] = s[1][i];
            s[2][i+1] = s[2][i]+1;
        }
    }
    for &(l, r) in &lrq {
        let mut ans = vec![];
        let g = s[0][r]-s[0][l-1];
        let c = s[1][r]-s[1][l-1];
        let p = s[2][r]-s[2][l-1];
        // println!("g {} c {} p {}", g, c, p);
        if judge(g+1, c, p, 0) {
            ans.push('G');
        }
        if judge(g, c+1, p, 1) {
            ans.push('C');
        }
        if judge(g, c, p+1, 2) {
            ans.push('P');
        }
        println!("{}", if ans.len() == 0 { "-1".to_string() } else { ans.iter().join(" ") });
    }
    
}

fn judge(g: i32, c: i32, p: i32, me: i32) -> bool {
    if me == 0 {
        if c == 0 && p == 0 {
            return true;
        }
        if c >= g && p > g {
            return true;
        }
        if c == 0 {
            return p > g;
        }
        if p == 0 {
            return c >= g;
        }
        
        return false;
    }
    if me == 1 {
        if g == 0 && p == 0 {
            return true;
        }
        if g > c && p >= c {
            return true;
        }
        if g == 0 {
            return p >= c;
        }
        if p == 0 {
            return g > c;
        }
        return false;
    }
    if me == 2 {
        if c == 0 && g == 0 {
            return true;
        }
        if g >= p && c > p {
            return true;
        }
        if g == 0 {
            return c > p;
        }
        if c == 0 {
            return g >= p;
        }
        return false;
    }
    return false;
}
```